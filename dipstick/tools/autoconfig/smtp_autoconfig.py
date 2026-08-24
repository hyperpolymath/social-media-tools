"""
SMTP Autoconfiguration Module
Automatically discovers and configures SMTP settings
"""

import asyncio
import smtplib
import dns.resolver
from typing import Optional, Dict, Tuple
from dataclasses import dataclass
from email.mime.text import MIMEText
import socket

@dataclass
class SMTPConfig:
    host: str
    port: int
    use_tls: bool
    use_ssl: bool
    auth_required: bool
    username: Optional[str] = None
    password: Optional[str] = None

class SMTPAutoConfig:
    """Automatically discover and configure SMTP settings"""

    # Common SMTP configurations
    COMMON_CONFIGS = [
        # Gmail
        {"host": "smtp.gmail.com", "port": 587, "use_tls": True, "use_ssl": False},
        {"host": "smtp.gmail.com", "port": 465, "use_tls": False, "use_ssl": True},
        # Outlook/Office365
        {"host": "smtp.office365.com", "port": 587, "use_tls": True, "use_ssl": False},
        # Yahoo
        {"host": "smtp.mail.yahoo.com", "port": 587, "use_tls": True, "use_ssl": False},
        # Generic
        {"host": "smtp.{domain}", "port": 587, "use_tls": True, "use_ssl": False},
        {"host": "mail.{domain}", "port": 587, "use_tls": True, "use_ssl": False},
    ]

    @staticmethod
    async def discover_smtp(email: str) -> Optional[SMTPConfig]:
        """
        Automatically discover SMTP settings for an email address

        Args:
            email: Email address to discover settings for

        Returns:
            SMTPConfig if successful, None otherwise
        """
        domain = email.split('@')[1]

        # Try MX record lookup
        mx_host = await SMTPAutoConfig._get_mx_record(domain)
        if mx_host:
            config = await SMTPAutoConfig._test_smtp_connection(mx_host)
            if config:
                return config

        # Try common configurations
        for template in SMTPAutoConfig.COMMON_CONFIGS:
            host = template["host"].format(domain=domain)
            config = await SMTPAutoConfig._test_smtp_connection(
                host,
                template["port"],
                template["use_tls"],
                template["use_ssl"]
            )
            if config:
                return config

        return None

    @staticmethod
    async def _get_mx_record(domain: str) -> Optional[str]:
        """Get MX record for domain"""
        try:
            answers = dns.resolver.resolve(domain, 'MX')
            # Return highest priority MX record
            mx_records = sorted([(r.preference, str(r.exchange)) for r in answers])
            return mx_records[0][1].rstrip('.') if mx_records else None
        except Exception:
            return None

    @staticmethod
    async def _test_smtp_connection(
        host: str,
        port: int = 587,
        use_tls: bool = True,
        use_ssl: bool = False,
        timeout: int = 10
    ) -> Optional[SMTPConfig]:
        """Test SMTP connection"""
        try:
            if use_ssl:
                server = smtplib.SMTP_SSL(host, port, timeout=timeout)
            else:
                server = smtplib.SMTP(host, port, timeout=timeout)
                if use_tls:
                    server.starttls()

            # Test if auth is required
            try:
                server.noop()
                auth_required = False
            except smtplib.SMTPException:
                auth_required = True

            server.quit()

            return SMTPConfig(
                host=host,
                port=port,
                use_tls=use_tls,
                use_ssl=use_ssl,
                auth_required=auth_required
            )
        except Exception:
            return None

    @staticmethod
    async def test_credentials(
        config: SMTPConfig,
        username: str,
        password: str
    ) -> bool:
        """Test SMTP credentials"""
        try:
            if config.use_ssl:
                server = smtplib.SMTP_SSL(config.host, config.port, timeout=10)
            else:
                server = smtplib.SMTP(config.host, config.port, timeout=10)
                if config.use_tls:
                    server.starttls()

            server.login(username, password)
            server.quit()
            return True
        except Exception:
            return False

    @staticmethod
    async def send_test_email(
        config: SMTPConfig,
        from_email: str,
        to_email: str,
        subject: str = "Test Email",
        body: str = "This is a test email from NUJ Monitor"
    ) -> bool:
        """Send a test email"""
        try:
            msg = MIMEText(body)
            msg['Subject'] = subject
            msg['From'] = from_email
            msg['To'] = to_email

            if config.use_ssl:
                server = smtplib.SMTP_SSL(config.host, config.port, timeout=10)
            else:
                server = smtplib.SMTP(config.host, config.port, timeout=10)
                if config.use_tls:
                    server.starttls()

            if config.auth_required and config.username and config.password:
                server.login(config.username, config.password)

            server.send_message(msg)
            server.quit()
            return True
        except Exception as e:
            print(f"Test email failed: {e}")
            return False

# Interactive autoconfiguration
async def interactive_smtp_setup() -> Optional[SMTPConfig]:
    """Interactive SMTP configuration wizard"""
    print("🔧 SMTP Autoconfiguration Wizard")
    print("=" * 50)

    email = input("\nEnter your email address: ").strip()
    if not email or '@' not in email:
        print("❌ Invalid email address")
        return None

    print(f"\n🔍 Discovering SMTP settings for {email}...")
    config = await SMTPAutoConfig.discover_smtp(email)

    if not config:
        print("❌ Could not automatically discover SMTP settings")
        print("\nPlease enter manually:")
        host = input("SMTP Host: ").strip()
        port = int(input("SMTP Port (587): ").strip() or "587")
        use_tls = input("Use TLS? (y/n): ").lower() == 'y'
        use_ssl = input("Use SSL? (y/n): ").lower() == 'y'

        config = SMTPConfig(
            host=host,
            port=port,
            use_tls=use_tls,
            use_ssl=use_ssl,
            auth_required=True
        )

    print(f"\n✅ Found SMTP server:")
    print(f"   Host: {config.host}")
    print(f"   Port: {config.port}")
    print(f"   TLS: {config.use_tls}")
    print(f"   SSL: {config.use_ssl}")

    if config.auth_required:
        print(f"\n🔐 Authentication required")
        username = input("Username (or press Enter to use email): ").strip() or email
        password = input("Password: ").strip()

        config.username = username
        config.password = password

        print("\n🧪 Testing credentials...")
        if await SMTPAutoConfig.test_credentials(config, username, password):
            print("✅ Credentials valid")
        else:
            print("❌ Authentication failed")
            return None

    # Test email
    send_test = input("\nSend test email? (y/n): ").lower() == 'y'
    if send_test:
        test_to = input("Send to (press Enter for same address): ").strip() or email
        print(f"\n📧 Sending test email to {test_to}...")
        if await SMTPAutoConfig.send_test_email(config, email, test_to):
            print("✅ Test email sent successfully")
        else:
            print("❌ Failed to send test email")

    # Save configuration
    print(f"\n💾 Configuration complete!")
    print(f"\nAdd to .env:")
    print(f"SMTP_HOST={config.host}")
    print(f"SMTP_PORT={config.port}")
    print(f"SMTP_USER={config.username or email}")
    print(f"SMTP_PASSWORD=***")
    print(f"SMTP_FROM={email}")
    if config.use_tls:
        print(f"SMTP_TLS=true")
    if config.use_ssl:
        print(f"SMTP_SSL=true")

    return config

if __name__ == "__main__":
    asyncio.run(interactive_smtp_setup())
