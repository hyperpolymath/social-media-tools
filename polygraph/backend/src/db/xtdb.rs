use anyhow::Result;
use reqwest::Client;

#[derive(Clone)]
pub struct XtdbClient {
    client: Client,
    base_url: String,
}

impl XtdbClient {
    pub async fn new(url: &str) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: url.to_string(),
        })
    }

    pub async fn put(&self, doc: serde_json::Value) -> Result<serde_json::Value> {
        let res = self
            .client
            .post(format!("{}/_xtdb/submit-tx", self.base_url))
            .json(&serde_json::json!({
                "tx-ops": [["put", doc]]
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn get(&self, id: &str) -> Result<Option<serde_json::Value>> {
        let res = self
            .client
            .get(format!("{}/_xtdb/entity", self.base_url))
            .query(&[("eid", id)])
            .send()
            .await?;

        if res.status().is_success() {
            Ok(Some(res.json().await?))
        } else {
            Ok(None)
        }
    }
}
