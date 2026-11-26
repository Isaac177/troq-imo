use std::collections::BTreeSet;

use anyhow::{bail, Result};
use serde_json::{json, Value};

use crate::audience::Audience;

const METHODS: [&str; 8] = ["get", "put", "post", "delete", "options", "head", "patch", "trace"];

pub fn finish(document: &mut Value) -> Result<()> {
    document["components"]["securitySchemes"]["bearer"] =
        json!({ "type": "http", "scheme": "bearer", "bearerFormat": "JWT" });
    let mut problems = Vec::new();
    let mut ids = BTreeSet::new();
    let Some(paths) = document.get_mut("paths").and_then(Value::as_object_mut) else {
        return Ok(());
    };
    for (path, item) in paths.iter_mut() {
        let audience = Audience::of_path(path);
        if audience.is_none() {
            problems.push(format!("{path}: outside the audience prefixes"));
        }
        let Some(item) = item.as_object_mut() else {
            continue;
        };
        for method in METHODS {
            let Some(operation) = item.get_mut(method) else {
                continue;
            };
            match operation.get("operationId").and_then(Value::as_str) {
                None => problems.push(format!("{method} {path}: operation_id missing")),
                Some(id) if !ids.insert(id.to_string()) => {
                    problems.push(format!("{method} {path}: operation_id {id} used twice"))
                }
                Some(_) => {}
            }
            if audience.is_some_and(Audience::requires_auth) {
                operation["security"] = json!([{ "bearer": [] }]);
            }
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        bail!("invalid api document:\n{}", problems.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(path: &str, id: Option<&str>) -> Value {
        let mut operation = json!({ "responses": {} });
        if let Some(id) = id {
            operation["operationId"] = json!(id);
        }
        json!({ "paths": { path: { "get": operation } } })
    }

    #[test]
    fn authenticated_audiences_require_the_bearer_scheme() {
        let mut host = document("/api/host/units", Some("units_list"));
        finish(&mut host).unwrap();
        assert_eq!(host["paths"]["/api/host/units"]["get"]["security"][0]["bearer"], json!([]));
        assert_eq!(host["components"]["securitySchemes"]["bearer"]["scheme"], "bearer");
        let mut public = document("/api/public/health", Some("health_get"));
        finish(&mut public).unwrap();
        assert!(public["paths"]["/api/public/health"]["get"].get("security").is_none());
    }

    #[test]
    fn rejects_paths_outside_the_audiences_and_missing_ids() {
        assert!(finish(&mut document("/health", Some("health_get"))).is_err());
        assert!(finish(&mut document("/api/guest/trips", None)).is_err());
    }

    #[test]
    fn rejects_duplicate_operation_ids() {
        let mut doc = document("/api/guest/trips", Some("same"));
        doc["paths"]["/api/guest/stays"] = doc["paths"]["/api/guest/trips"].clone();
        assert!(finish(&mut doc).is_err());
    }
}
