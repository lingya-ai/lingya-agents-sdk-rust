use std::fs;
use std::path::PathBuf;

use lingya_agents_sdk::client::sign_canonical;
use serde::Deserialize;

#[derive(Deserialize)]
struct Vectors {
    cases: Vec<Vector>,
}

#[derive(Deserialize)]
struct Vector {
    secret: String,
    canonical: String,
    signature: String,
}

#[test]
fn matches_every_published_hmac_vector() {
    let path = contract_root().join("test-vectors/hmac-v1.json");
    let vectors: Vectors = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    for vector in vectors.cases {
        assert_eq!(
            sign_canonical(&vector.secret, &vector.canonical).unwrap(),
            vector.signature
        );
    }
}

#[test]
fn production_api_and_models_do_not_use_dynamic_json_or_maps() {
    let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for path in rust_sources(&source_root) {
        let source = fs::read_to_string(&path).unwrap();
        assert!(
            !source.contains("serde_json::Value"),
            "{} uses dynamic JSON",
            path.display()
        );
        assert!(
            !source.contains("HashMap<"),
            "{} uses a map model",
            path.display()
        );
    }
}

fn rust_sources(directory: &std::path::Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            paths.extend(rust_sources(&path));
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            paths.push(path);
        }
    }
    paths
}

fn contract_root() -> PathBuf {
    std::env::var_os("LINGYA_CONTRACT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../lingya-agents-openapi")
        })
}
