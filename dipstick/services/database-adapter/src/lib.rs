// SPDX-License-Identifier: MPL-2.0
// NUJ Social Media Ethics Monitor - Virtuoso Triple Store Adapter
// Purpose: RDF/SPARQL interface for semantic policy tracking
// Tech: Rust + reqwest (SPARQL protocol over HTTPS)
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>
//
// Every query is a plain SPARQL string POSTed to the Virtuoso /sparql
// endpoint. Values that go into a query are escaped as SPARQL string
// literals (`literal`) or checked as IRIs (`iri`), so caller data cannot
// change the shape of a query.

use std::fmt::Display;

use chrono::{DateTime, Duration, SecondsFormat, TimeZone, Utc};
use serde_json::{Map, Value};

/// NUJ Monitor ontology namespace.
pub const NUJ: &str = "https://nuj.org.uk/monitor/ontology/";
pub const PLATFORM: &str = "https://nuj.org.uk/monitor/platform/";
pub const POLICY: &str = "https://nuj.org.uk/monitor/policy/";
pub const CHANGE: &str = "https://nuj.org.uk/monitor/change/";
/// W3C vocabulary identifiers (names, not fetched URLs).
pub const RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

pub const PLATFORMS_GRAPH: &str = "https://nuj.org.uk/monitor/platforms";
pub const CHANGES_GRAPH: &str = "https://nuj.org.uk/monitor/changes";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("not a valid IRI: {0:?}")]
    InvalidIri(String),
    #[error("not a finite number: {0}")]
    InvalidNumber(f64),
    #[error("semantic search needs at least one keyword")]
    NoKeywords,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct VirtuosoConfig {
    pub host: String,
    pub port: u16,
    pub http_port: u16,
    pub username: String,
    pub password: String,
    pub default_graph: String,
}

impl Default for VirtuosoConfig {
    /// Virtuoso's stock local settings. Override `username`/`password`
    /// for anything that is not a throwaway local instance.
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 1111,
            http_port: 8890,
            username: "dba".into(),
            password: "dba".into(),
            default_graph: "https://nuj.org.uk/monitor/default-graph".into(),
        }
    }
}

/// A SPARQL string literal, with `\`, `"` and line breaks escaped.
pub fn literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `<base + local>` as a SPARQL IRI reference; refuses characters that
/// IRIREF does not allow, as rdflib's `URIRef.n3()` did.
pub fn iri(base: &str, local: &str) -> Result<String> {
    let full = format!("{base}{local}");
    let bad =
        |c: char| c <= ' ' || matches!(c, '<' | '>' | '"' | '{' | '}' | '|' | '^' | '`' | '\\');
    if full.is_empty() || full.chars().any(bad) {
        return Err(Error::InvalidIri(full));
    }
    Ok(format!("<{full}>"))
}

/// A SPARQL numeric literal (finite only).
fn number(x: f64) -> Result<String> {
    if !x.is_finite() {
        return Err(Error::InvalidNumber(x));
    }
    Ok(format!("{x:?}"))
}

/// A naive UTC timestamp, the form the original adapter wrote
/// (`datetime.utcnow().isoformat()`).
fn utc_naive(t: DateTime<Utc>) -> String {
    t.naive_utc().format("%Y-%m-%dT%H:%M:%S%.6f").to_string()
}

fn datetime<Tz: TimeZone>(t: &DateTime<Tz>) -> String
where
    Tz::Offset: Display,
{
    t.to_rfc3339_opts(SecondsFormat::AutoSi, false)
}

// ---------------------------------------------------------------------------
// Query builders (pure; the adapter methods below send them)
// ---------------------------------------------------------------------------

pub fn insert_platform_query(
    platform_id: &str,
    name: &str,
    display_name: &str,
    api_enabled: bool,
    monitoring_active: bool,
    created_at: DateTime<Utc>,
) -> Result<String> {
    let platform_uri = iri(PLATFORM, platform_id)?;
    Ok(format!(
        r#"
        PREFIX nuj: <{NUJ}>
        PREFIX platform: <{PLATFORM}>
        PREFIX rdfs: <{RDFS}>
        PREFIX xsd: <{XSD}>

        INSERT DATA {{
          GRAPH <{PLATFORMS_GRAPH}> {{
            {platform_uri} a nuj:Platform ;
              rdfs:label {label} ;
              nuj:name {name} ;
              nuj:apiEnabled {api_enabled} ;
              nuj:monitoringActive {monitoring_active} ;
              nuj:createdAt "{created}"^^xsd:dateTime .
          }}
        }}
        "#,
        label = literal(display_name),
        name = literal(name),
        created = utc_naive(created_at),
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn insert_policy_change_query<Tz: TimeZone>(
    change_id: &str,
    platform_id: &str,
    policy_document_id: &str,
    severity: &str,
    confidence_score: f64,
    change_summary: &str,
    requires_notification: bool,
    detected_at: &DateTime<Tz>,
) -> Result<String>
where
    Tz::Offset: Display,
{
    let change_uri = iri(CHANGE, change_id)?;
    let platform_uri = iri(PLATFORM, platform_id)?;
    let policy_uri = iri(POLICY, policy_document_id)?;
    Ok(format!(
        r#"
        PREFIX nuj: <{NUJ}>
        PREFIX change: <{CHANGE}>
        PREFIX policy: <{POLICY}>
        PREFIX platform: <{PLATFORM}>
        PREFIX xsd: <{XSD}>

        INSERT DATA {{
          GRAPH <{CHANGES_GRAPH}> {{
            {change_uri} a nuj:PolicyChange ;
              nuj:affectsPlatform {platform_uri} ;
              nuj:affectsPolicy {policy_uri} ;
              nuj:severity {severity} ;
              nuj:confidenceScore {confidence} ;
              nuj:changeSummary {summary} ;
              nuj:requiresNotification {requires_notification} ;
              nuj:detectedAt "{detected}"^^xsd:dateTime .
          }}
        }}
        "#,
        severity = literal(severity),
        confidence = number(confidence_score)?,
        summary = literal(change_summary),
        detected = datetime(detected_at),
    ))
}

pub fn recent_changes_query(
    days: i64,
    severity: Option<&str>,
    platform: Option<&str>,
    now: DateTime<Utc>,
) -> String {
    let severity_filter = severity
        .filter(|s| !s.is_empty())
        .map(|s| format!("FILTER (?severity = {})", literal(s)))
        .unwrap_or_default();
    let platform_filter = platform
        .filter(|s| !s.is_empty())
        .map(|s| format!("FILTER (?platformLabel = {})", literal(s)))
        .unwrap_or_default();
    let since = utc_naive(now - Duration::days(days));
    format!(
        r#"
        PREFIX nuj: <{NUJ}>
        PREFIX rdfs: <{RDFS}>
        PREFIX xsd: <{XSD}>

        SELECT ?change ?platform ?platformLabel ?severity ?confidence ?summary ?detectedAt
        WHERE {{
          GRAPH <{CHANGES_GRAPH}> {{
            ?change a nuj:PolicyChange ;
              nuj:affectsPlatform ?platform ;
              nuj:severity ?severity ;
              nuj:confidenceScore ?confidence ;
              nuj:changeSummary ?summary ;
              nuj:detectedAt ?detectedAt .

            ?platform rdfs:label ?platformLabel .

            FILTER (?detectedAt > "{since}"^^xsd:dateTime)
            {severity_filter}
            {platform_filter}
          }}
        }}
        ORDER BY DESC(?detectedAt)
        LIMIT 100
        "#
    )
}

pub fn platform_statistics_query(platform_id: &str) -> Result<String> {
    let platform_uri = iri(PLATFORM, platform_id)?;
    Ok(format!(
        r#"
        PREFIX nuj: <{NUJ}>

        SELECT
          (COUNT(DISTINCT ?change) as ?totalChanges)
          (COUNT(DISTINCT ?criticalChange) as ?criticalChanges)
          (AVG(?confidence) as ?avgConfidence)
        WHERE {{
          {{
            SELECT ?change ?confidence WHERE {{
              GRAPH <{CHANGES_GRAPH}> {{
                ?change a nuj:PolicyChange ;
                  nuj:affectsPlatform {platform_uri} ;
                  nuj:confidenceScore ?confidence .
              }}
            }}
          }}
          OPTIONAL {{
            ?criticalChange nuj:severity "critical" ;
              nuj:affectsPlatform {platform_uri} .
          }}
        }}
        "#
    ))
}

pub fn semantic_search_query<S: AsRef<str>>(keywords: &[S]) -> Result<String> {
    if keywords.is_empty() {
        return Err(Error::NoKeywords);
    }
    let keyword_filter = keywords
        .iter()
        .map(|kw| format!("CONTAINS(?summary, {})", literal(kw.as_ref())))
        .collect::<Vec<_>>()
        .join(" || ");
    Ok(format!(
        r#"
        PREFIX nuj: <{NUJ}>
        PREFIX rdfs: <{RDFS}>

        SELECT ?change ?platform ?severity ?summary ?detectedAt
        WHERE {{
          GRAPH <{CHANGES_GRAPH}> {{
            ?change a nuj:PolicyChange ;
              nuj:affectsPlatform ?platform ;
              nuj:severity ?severity ;
              nuj:changeSummary ?summary ;
              nuj:detectedAt ?detectedAt .

            FILTER ({keyword_filter})
          }}

          ?platform rdfs:label ?platformLabel .
        }}
        ORDER BY DESC(?detectedAt)
        LIMIT 50
        "#
    ))
}

pub fn export_graph_query(graph_uri: &str) -> Result<String> {
    let graph = iri("", graph_uri)?;
    Ok(format!(
        r#"
        CONSTRUCT {{ ?s ?p ?o }}
        WHERE {{
          GRAPH {graph} {{ ?s ?p ?o }}
        }}
        "#
    ))
}

fn bindings(result: Value) -> Vec<Value> {
    match result.get("results").and_then(|r| r.get("bindings")) {
        Some(Value::Array(rows)) => rows.clone(),
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

/// Adapter for the Virtuoso RDF triple store.
pub struct VirtuosoAdapter {
    pub config: VirtuosoConfig,
    pub sparql_endpoint: String,
    client: reqwest::Client,
}

impl VirtuosoAdapter {
    /// Talks to `https://{host}:{http_port}/sparql`.
    pub fn new(config: VirtuosoConfig) -> Self {
        let endpoint = format!("https://{}:{}/sparql", config.host, config.http_port);
        Self::with_endpoint(config, endpoint)
    }

    /// Talks to an explicit SPARQL endpoint (a proxy, or a loopback test
    /// server).
    pub fn with_endpoint(config: VirtuosoConfig, sparql_endpoint: impl Into<String>) -> Self {
        Self {
            config,
            sparql_endpoint: sparql_endpoint.into(),
            client: reqwest::Client::new(),
        }
    }

    async fn post(&self, form: &[(&str, &str)]) -> Result<reqwest::Response> {
        Ok(self
            .client
            .post(&self.sparql_endpoint)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .form(form)
            .send()
            .await?)
    }

    /// Execute a SPARQL query and return the JSON result. `extra` adds
    /// further form fields to the request.
    pub async fn execute_sparql(&self, query: &str, extra: &[(&str, &str)]) -> Result<Value> {
        let mut form = vec![
            ("query", query),
            ("format", "application/sparql-results+json"),
        ];
        form.extend_from_slice(extra);
        let response = self.post(&form).await?.error_for_status()?;
        Ok(response.json().await?)
    }

    /// Insert a platform as RDF triples. `policy_urls` is accepted for
    /// interface compatibility and, as before, not stored.
    pub async fn insert_platform(
        &self,
        platform_id: &str,
        name: &str,
        display_name: &str,
        api_enabled: bool,
        monitoring_active: bool,
        _policy_urls: &[String],
    ) -> Result<bool> {
        let query = insert_platform_query(
            platform_id,
            name,
            display_name,
            api_enabled,
            monitoring_active,
            Utc::now(),
        )?;
        self.execute_sparql(&query, &[]).await?;
        Ok(true)
    }

    /// Insert a policy change as RDF.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_policy_change<Tz: TimeZone>(
        &self,
        change_id: &str,
        platform_id: &str,
        policy_document_id: &str,
        severity: &str,
        confidence_score: f64,
        change_summary: &str,
        requires_notification: bool,
        detected_at: &DateTime<Tz>,
    ) -> Result<bool>
    where
        Tz::Offset: Display,
    {
        let query = insert_policy_change_query(
            change_id,
            platform_id,
            policy_document_id,
            severity,
            confidence_score,
            change_summary,
            requires_notification,
            detected_at,
        )?;
        self.execute_sparql(&query, &[]).await?;
        Ok(true)
    }

    /// Policy changes detected in the last `days` days, newest first (at
    /// most 100).
    pub async fn query_recent_changes(
        &self,
        days: i64,
        severity: Option<&str>,
        platform: Option<&str>,
    ) -> Result<Vec<Value>> {
        let query = recent_changes_query(days, severity, platform, Utc::now());
        Ok(bindings(self.execute_sparql(&query, &[]).await?))
    }

    /// Change counts and mean confidence for one platform; an empty map
    /// when there is no row.
    pub async fn query_platform_statistics(&self, platform_id: &str) -> Result<Value> {
        let query = platform_statistics_query(platform_id)?;
        let rows = bindings(self.execute_sparql(&query, &[]).await?);
        Ok(rows
            .into_iter()
            .next()
            .unwrap_or_else(|| Value::Object(Map::new())))
    }

    /// Changes whose summary contains any of the keywords (at most 50).
    pub async fn semantic_search<S: AsRef<str>>(&self, keywords: &[S]) -> Result<Vec<Value>> {
        let query = semantic_search_query(keywords)?;
        Ok(bindings(self.execute_sparql(&query, &[]).await?))
    }

    /// Export a whole named graph, as `application/{format}` (e.g.
    /// "turtle"). Like the original, the body is returned whatever the
    /// HTTP status.
    pub async fn export_graph(&self, graph_uri: &str, format: &str) -> Result<String> {
        let query = export_graph_query(graph_uri)?;
        let fmt = format!("application/{format}");
        Ok(self
            .post(&[("query", &query), ("format", &fmt)])
            .await?
            .text()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::FixedOffset;

    #[test]
    fn literal_escapes_quotes_backslashes_and_breaks() {
        assert_eq!(literal(r#"a "b" \c"#), r#""a \"b\" \\c""#);
        assert_eq!(literal("x\ny\tz\r"), r#""x\ny\tz\r""#);
        // An attempt to close the literal and inject a triple stays inside it.
        let evil = r#"x" ; nuj:evil "y"#;
        assert_eq!(literal(evil), r#""x\" ; nuj:evil \"y""#);
    }

    #[test]
    fn iri_rejects_characters_that_would_break_out() {
        assert_eq!(
            iri(PLATFORM, "twitter").unwrap(),
            "<https://nuj.org.uk/monitor/platform/twitter>"
        );
        for bad in [
            "a b", "a>b", "a<b", "a\"b", "a{b", "a}b", "a|b", "a^b", "a`b", "a\\b", "a\nb",
        ] {
            assert!(iri(PLATFORM, bad).is_err(), "{bad:?} accepted");
        }
    }

    #[test]
    fn insert_platform_query_shape() {
        let t = Utc.with_ymd_and_hms(2026, 1, 2, 3, 4, 5).unwrap();
        let q = insert_platform_query("x", "X", "The \"X\"", true, false, t).unwrap();
        assert!(q.contains("<https://nuj.org.uk/monitor/platform/x> a nuj:Platform ;"));
        assert!(q.contains(r#"rdfs:label "The \"X\"" ;"#));
        assert!(q.contains("nuj:apiEnabled true ;"));
        assert!(q.contains("nuj:monitoringActive false ;"));
        assert!(q.contains(r#"nuj:createdAt "2026-01-02T03:04:05.000000"^^xsd:dateTime ."#));
        assert!(q.contains("PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>"));
    }

    #[test]
    fn insert_policy_change_query_shape() {
        let t = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 1, 2, 3, 4, 5)
            .unwrap();
        let q =
            insert_policy_change_query("c1", "x", "tos", "critical", 0.85, "said \"no\"", true, &t)
                .unwrap();
        assert!(q.contains("nuj:affectsPolicy <https://nuj.org.uk/monitor/policy/tos> ;"));
        assert!(q.contains("nuj:confidenceScore 0.85 ;"));
        assert!(q.contains(r#"nuj:changeSummary "said \"no\"" ;"#));
        assert!(q.contains(r#"nuj:detectedAt "2026-01-02T03:04:05+00:00"^^xsd:dateTime ."#));
        assert!(insert_policy_change_query("c", "x", "p", "low", f64::NAN, "", false, &t).is_err());
    }

    #[test]
    fn recent_changes_filters() {
        let now = Utc.with_ymd_and_hms(2026, 1, 31, 0, 0, 0).unwrap();
        let q = recent_changes_query(30, Some("critical"), None, now);
        assert!(q.contains(r#"FILTER (?detectedAt > "2026-01-01T00:00:00.000000"^^xsd:dateTime)"#));
        assert!(q.contains(r#"FILTER (?severity = "critical")"#));
        assert!(!q.contains("?platformLabel = "));
        let q = recent_changes_query(7, None, Some("X"), now);
        assert!(!q.contains("?severity = "));
        assert!(q.contains(r#"FILTER (?platformLabel = "X")"#));
    }

    #[test]
    fn semantic_search_joins_keywords_with_or() {
        let q = semantic_search_query(&["journalist", "content removal"]).unwrap();
        assert!(q.contains(
            r#"FILTER (CONTAINS(?summary, "journalist") || CONTAINS(?summary, "content removal"))"#
        ));
        assert!(matches!(
            semantic_search_query::<&str>(&[]),
            Err(Error::NoKeywords)
        ));
    }

    #[test]
    fn export_graph_rejects_bad_graph_iri() {
        assert!(export_graph_query(CHANGES_GRAPH)
            .unwrap()
            .contains("GRAPH <https://nuj.org.uk/monitor/changes>"));
        assert!(export_graph_query("x> } DROP ALL #").is_err());
    }
}
