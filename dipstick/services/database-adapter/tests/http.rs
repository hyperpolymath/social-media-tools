// SPDX-License-Identifier: MPL-2.0
// The adapter against a loopback SPARQL endpoint: checks the request it
// sends (method, basic auth, form fields) and how it reads the reply.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;

use nuj_virtuoso_adapter::{VirtuosoAdapter, VirtuosoConfig};

/// Serve one request with `status`/`body`; return the raw request.
fn one_shot(status: &'static str, body: &'static str) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/sparql", listener.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut head = String::new();
        let mut len = 0usize;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                len = v.trim().parse().unwrap();
            }
            head.push_str(&line);
            if line == "\r\n" {
                break;
            }
        }
        let mut body_in = vec![0u8; len];
        reader.read_exact(&mut body_in).unwrap();
        let mut stream = stream;
        write!(
            stream,
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        head + &String::from_utf8(body_in).unwrap()
    });
    (url, handle)
}

fn form_value(request: &str, key: &str) -> String {
    let body = request.split("\r\n\r\n").nth(1).unwrap();
    for pair in body.split('&') {
        let (k, v) = pair.split_once('=').unwrap();
        if k == key {
            let v = v.replace('+', " ");
            let mut out = Vec::new();
            let b = v.as_bytes();
            let mut i = 0;
            while i < b.len() {
                if b[i] == b'%' {
                    out.push(u8::from_str_radix(&v[i + 1..i + 3], 16).unwrap());
                    i += 3;
                } else {
                    out.push(b[i]);
                    i += 1;
                }
            }
            return String::from_utf8(out).unwrap();
        }
    }
    panic!("no {key} in form");
}

#[tokio::test]
async fn recent_changes_posts_query_and_reads_bindings() {
    let (url, server) = one_shot(
        "200 OK",
        r#"{"results":{"bindings":[{"change":{"type":"uri","value":"https://nuj.org.uk/monitor/change/1"}}]}}"#,
    );
    let adapter = VirtuosoAdapter::with_endpoint(VirtuosoConfig::default(), url);
    let rows = adapter
        .query_recent_changes(7, Some("critical"), None)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);

    let req = server.join().unwrap();
    assert!(req.starts_with("POST /sparql HTTP/1.1\r\n"));
    // dba:dba
    assert!(req
        .to_ascii_lowercase()
        .contains("authorization: basic zgjhomriyq=="));
    assert_eq!(
        form_value(&req, "format"),
        "application/sparql-results+json"
    );
    let q = form_value(&req, "query");
    assert!(q.contains(r#"FILTER (?severity = "critical")"#));
    assert!(q.contains("LIMIT 100"));
}

#[tokio::test]
async fn statistics_without_rows_is_empty_object() {
    let (url, server) = one_shot("200 OK", r#"{"results":{"bindings":[]}}"#);
    let adapter = VirtuosoAdapter::with_endpoint(VirtuosoConfig::default(), url);
    let stats = adapter.query_platform_statistics("twitter").await.unwrap();
    assert_eq!(stats, serde_json::json!({}));
    server.join().unwrap();
}

#[tokio::test]
async fn http_error_is_an_error() {
    let (url, server) = one_shot("500 Internal Server Error", "{}");
    let adapter = VirtuosoAdapter::with_endpoint(VirtuosoConfig::default(), url);
    assert!(adapter.semantic_search(&["x"]).await.is_err());
    server.join().unwrap();
}

#[tokio::test]
async fn export_graph_returns_body_whatever_the_status() {
    let (url, server) = one_shot("404 Not Found", "not here");
    let adapter = VirtuosoAdapter::with_endpoint(VirtuosoConfig::default(), url);
    let text = adapter
        .export_graph("https://nuj.org.uk/monitor/changes", "turtle")
        .await
        .unwrap();
    assert_eq!(text, "not here");
    let req = server.join().unwrap();
    assert_eq!(form_value(&req, "format"), "application/turtle");
}
