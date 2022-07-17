use assert_cmd::Command;
use fj_host::Parameters;
use std::{collections::HashMap, path::Path};

#[test]
fn load_the_cuboid_model_through_fornjot() {
    // Compile cuboid.wasm
    Command::new(env!("CARGO"))
        .args([
            "build",
            "--package=cuboid",
            "--target=wasm32-unknown-unknown",
            "--quiet",
        ])
        .assert()
        .success();

    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project_root = crate_dir.ancestors().nth(2).unwrap();
    let target_dir = dbg!(project_root.join("target"));
    let binary = target_dir
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join("cuboid.wasm");
    assert!(binary.exists(), "{binary:?} should exist");
    std::env::set_var(fj_wasm_shim::ENV_NAME, &binary);

    let model = fj_host::Model::from_path(crate_dir.to_path_buf(), Some(target_dir)).unwrap();

    let mut params = HashMap::new();
    params.insert("x".to_string(), "42.0".to_string());
    let params = Parameters(params);
    // TODO: Assert that the shape is as expected
    // See https://github.com/hannobraun/Fornjot/issues/831
    let _shape = model.load_once(&params).unwrap();
}
