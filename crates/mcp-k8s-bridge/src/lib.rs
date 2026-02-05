use kube::Client;
use kube::core::ApiResource;
use kube::discovery;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrdSchema {
    pub group: String,
    pub version: String,
    pub kind: String,
    pub openapi_v3: serde_json::Value,
}

// MCP tool handler: "list_neuro_crd_schemas"
pub async fn list_neuro_crd_schemas() -> anyhow::Result<Vec<CrdSchema>> {
    let client = Client::try_default().await?;
    let discovery = discovery::Discovery::new(client).run().await?;
    let mut out = Vec::new();
    for group in discovery.groups() {
        if !group.name().starts_with("neuro.pc") {
            continue;
        }
        for (ar, caps) in group.recommended_resources() {
            if !caps.operations.supports_list {
                continue;
            }
            out.push(CrdSchema {
                group: group.name().to_string(),
                version: ar.version.clone(),
                kind: ar.kind.clone(),
                openapi_v3: serde_json::Value::Null, // fill from OpenAPI aggregation if needed
            });
        }
    }
    Ok(out)
}
