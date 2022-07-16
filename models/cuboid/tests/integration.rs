use std::{collections::HashMap, path::Path};

use fj_host::{Model, Parameters};

#[test]
fn load_native_cuboid_and_generate_geometry() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = crate_dir.parent().unwrap().parent().unwrap().join("target");

    let model = Model::from_path(crate_dir.into(), Some(target_dir.into())).unwrap();

    let mut arguments = HashMap::new();
    arguments.insert("x".to_string(), "10".to_string());
    let actual = model.load_once(&Parameters(arguments)).unwrap();

    let rectangle = fj::Sketch::from_points(vec![
        [-5.0, -1.0],
        [10.0 / 2., -2.0 / 2.],
        [10.0 / 2., 2.0 / 2.],
        [-10.0 / 2., 2.0 / 2.],
    ])
    .with_color([100, 255, 0, 200]);
    let expected = fj::Sweep::from_path(rectangle.into(), [0., 0., 3.0]);

    // TODO: Enable this assertion when we can compare shapes for equality.
    // https://github.com/hannobraun/Fornjot/issues/831

    // assert_eq!(actual, expected);
    let _ = (actual, expected);
}
