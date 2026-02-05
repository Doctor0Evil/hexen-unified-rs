use jsonschema::{Draft, JSONSchema};
use serde_json::Value;

pub fn validate_against_schema(instance: &Value, schema: &Value) -> anyhow::Result<()> {
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema)?;
    if let Err(errors) = compiled.validate(instance) {
        let msg = errors
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!("Schema validation failed: {}", msg);
    }
    Ok(())
}
