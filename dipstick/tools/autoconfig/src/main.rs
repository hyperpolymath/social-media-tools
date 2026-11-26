// SPDX-License-Identifier: MPL-2.0
// NUJ Social Media Ethics Monitor - SMTP Autoconfiguration
// Purpose: Discover SMTP settings for an email address and walk the user
//          through checking credentials and sending a test email.
// Tech: Rust + lettre (SMTP) + hickory-resolver (MX lookup)
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>
//
// Usage: cargo run --release   (or `just setup-smtp` from dipstick/)

use std::io::{self, BufRead, Write};
use std::process::ExitCode;
use std::time::Duration;

use hickory_resolver::proto::rr::RData;
use hickory_resolver::TokioResolver;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::AsyncSmtpTransportBuilder;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

const TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
struct SmtpConfig {
    host: String,
    port: u16,
    use_tls: bool,
    use_ssl: bool,
    auth_required: bool,
    username: Option<String>,
    password: Option<String>,
}

struct Template {
    host: &'static str,
    port: u16,
    use_tls: bool,
    use_ssl: bool,
}

/// Common SMTP configurations, tried in order after the MX host.
const COMMON_CONFIGS: &[Template] = &[
    // Gmail
    Template {
        host: "smtp.gmail.com",
        port: 587,
        use_tls: true,
        use_ssl: false,
    },
    Template {
        host: "smtp.gmail.com",
        port: 465,
        use_tls: false,
        use_ssl: true,
    },
    // Outlook/Office365
    Template {
        host: "smtp.office365.com",
        port: 587,
        use_tls: true,
        use_ssl: false,
    },
    // Yahoo
    Template {
        host: "smtp.mail.yahoo.com",
        port: 587,
        use_tls: true,
        use_ssl: false,
    },
    // Generic
    Template {
        host: "smtp.{domain}",
        port: 587,
        use_tls: true,
        use_ssl: false,
    },
    Template {
        host: "mail.{domain}",
        port: 587,
        use_tls: true,
        use_ssl: false,
    },
];

/// Build a transport for host/port: implicit TLS when `use_ssl`, required
/// STARTTLS when `use_tls`, plain otherwise.
fn transport(
    host: &str,
    port: u16,
    use_tls: bool,
    use_ssl: bool,
) -> Result<AsyncSmtpTransportBuilder, lettre::transport::smtp::Error> {
    let builder = if use_ssl {
        AsyncSmtpTransport::<Tokio1Executor>::relay(host)?
    } else if use_tls {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host)?
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
    };
    Ok(builder.port(port).timeout(Some(TIMEOUT)))
}

/// Highest-priority (lowest preference) MX host for a domain, without the
/// trailing dot.
async fn get_mx_record(domain: &str) -> Option<String> {
    let resolver = TokioResolver::builder_tokio().ok()?.build().ok()?;
    // Fully qualified, so no search domains are appended.
    let fqdn = if domain.ends_with('.') {
        domain.to_string()
    } else {
        format!("{domain}.")
    };
    let lookup = resolver.mx_lookup(fqdn.as_str()).await.ok()?;
    let mut records: Vec<(u16, String)> = lookup
        .answers()
        .iter()
        .filter_map(|r| match &r.data {
            RData::MX(mx) => Some((mx.preference, mx.exchange.to_string())),
            _ => None,
        })
        .collect();
    records.sort();
    records
        .into_iter()
        .next()
        .map(|(_, host)| host.trim_end_matches('.').to_string())
}

/// Connect (and STARTTLS where asked) and send NOOP. A connection failure
/// yields None; a refused NOOP marks the server as requiring auth.
async fn test_smtp_connection(
    host: &str,
    port: u16,
    use_tls: bool,
    use_ssl: bool,
) -> Option<SmtpConfig> {
    let smtp: AsyncSmtpTransport<Tokio1Executor> =
        transport(host, port, use_tls, use_ssl).ok()?.build();
    let noop_ok = smtp.test_connection().await.ok()?;
    Some(SmtpConfig {
        host: host.to_string(),
        port,
        use_tls,
        use_ssl,
        auth_required: !noop_ok,
        username: None,
        password: None,
    })
}

async fn discover_smtp(email: &str) -> Option<SmtpConfig> {
    let domain = email.split('@').nth(1).unwrap_or("");

    if let Some(mx_host) = get_mx_record(domain).await {
        if let Some(config) = test_smtp_connection(&mx_host, 587, true, false).await {
            return Some(config);
        }
    }

    for t in COMMON_CONFIGS {
        let host = t.host.replace("{domain}", domain);
        if let Some(config) = test_smtp_connection(&host, t.port, t.use_tls, t.use_ssl).await {
            return Some(config);
        }
    }
    None
}

/// Connect and log in with the given credentials.
async fn test_credentials(config: &SmtpConfig, username: &str, password: &str) -> bool {
    let Ok(builder) = transport(&config.host, config.port, config.use_tls, config.use_ssl) else {
        return false;
    };
    let smtp: AsyncSmtpTransport<Tokio1Executor> = builder
        .credentials(Credentials::new(username.to_string(), password.to_string()))
        .build();
    smtp.test_connection().await.is_ok()
}

async fn send_test_email(config: &SmtpConfig, from_email: &str, to_email: &str) -> bool {
    let subject = "Test Email";
    let body = "This is a test email from NUJ Monitor";
    let result = async {
        let msg = Message::builder()
            .from(from_email.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;
        let mut builder = transport(&config.host, config.port, config.use_tls, config.use_ssl)?;
        if config.auth_required {
            if let (Some(user), Some(pass)) = (&config.username, &config.password) {
                if !user.is_empty() && !pass.is_empty() {
                    builder = builder.credentials(Credentials::new(user.clone(), pass.clone()));
                }
            }
        }
        let smtp: AsyncSmtpTransport<Tokio1Executor> = builder.build();
        smtp.send(msg).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;
    match result {
        Ok(()) => true,
        Err(e) => {
            println!("Test email failed: {e}");
            false
        }
    }
}

/// Print a prompt and read one line (without its newline). EOF is an error.
fn input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut line = String::new();
    if io::stdin().lock().read_line(&mut line)? == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "EOF when reading a line",
        ));
    }
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    Ok(line)
}

/// Interactive SMTP configuration wizard.
async fn interactive_smtp_setup() -> io::Result<Option<SmtpConfig>> {
    println!("🔧 SMTP Autoconfiguration Wizard");
    println!("{}", "=".repeat(50));

    let email = input("\nEnter your email address: ")?.trim().to_string();
    if email.is_empty() || !email.contains('@') {
        println!("❌ Invalid email address");
        return Ok(None);
    }

    println!("\n🔍 Discovering SMTP settings for {email}...");
    let mut config = match discover_smtp(&email).await {
        Some(c) => c,
        None => {
            println!("❌ Could not automatically discover SMTP settings");
            println!("\nPlease enter manually:");
            let host = input("SMTP Host: ")?.trim().to_string();
            let port_s = input("SMTP Port (587): ")?.trim().to_string();
            let port_s = if port_s.is_empty() {
                "587".to_string()
            } else {
                port_s
            };
            let port: u16 = port_s.parse().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid port: '{port_s}'"),
                )
            })?;
            let use_tls = input("Use TLS? (y/n): ")?.to_lowercase() == "y";
            let use_ssl = input("Use SSL? (y/n): ")?.to_lowercase() == "y";
            SmtpConfig {
                host,
                port,
                use_tls,
                use_ssl,
                auth_required: true,
                username: None,
                password: None,
            }
        }
    };

    println!("\n✅ Found SMTP server:");
    println!("   Host: {}", config.host);
    println!("   Port: {}", config.port);
    println!("   TLS: {}", py_bool(config.use_tls));
    println!("   SSL: {}", py_bool(config.use_ssl));

    if config.auth_required {
        println!("\n🔐 Authentication required");
        let username = input("Username (or press Enter to use email): ")?
            .trim()
            .to_string();
        let username = if username.is_empty() {
            email.clone()
        } else {
            username
        };
        let password = input("Password: ")?.trim().to_string();

        config.username = Some(username.clone());
        config.password = Some(password.clone());

        println!("\n🧪 Testing credentials...");
        if test_credentials(&config, &username, &password).await {
            println!("✅ Credentials valid");
        } else {
            println!("❌ Authentication failed");
            return Ok(None);
        }
    }

    let send_test = input("\nSend test email? (y/n): ")?.to_lowercase() == "y";
    if send_test {
        let test_to = input("Send to (press Enter for same address): ")?
            .trim()
            .to_string();
        let test_to = if test_to.is_empty() {
            email.clone()
        } else {
            test_to
        };
        println!("\n📧 Sending test email to {test_to}...");
        if send_test_email(&config, &email, &test_to).await {
            println!("✅ Test email sent successfully");
        } else {
            println!("❌ Failed to send test email");
        }
    }

    println!("\n💾 Configuration complete!");
    println!("\nAdd to .env:");
    println!("SMTP_HOST={}", config.host);
    println!("SMTP_PORT={}", config.port);
    println!(
        "SMTP_USER={}",
        config
            .username
            .as_deref()
            .filter(|u| !u.is_empty())
            .unwrap_or(&email)
    );
    println!("SMTP_PASSWORD=***");
    println!("SMTP_FROM={email}");
    if config.use_tls {
        println!("SMTP_TLS=true");
    }
    if config.use_ssl {
        println!("SMTP_SSL=true");
    }

    Ok(Some(config))
}

/// Render a bool the way the original wizard did (True/False).
fn py_bool(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match interactive_smtp_setup().await {
        // As before: a wizard that gives up (bad address, failed auth) still
        // exits 0; only an input error exits non-zero.
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("\nsmtp-autoconfig: {e}");
            ExitCode::FAILURE
        }
    }
}
