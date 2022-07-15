use fornjot_plugins::prelude::*;

#[no_mangle] // TODO: hide this behind a custom attribute
pub fn fornjot_plugin_init(host: &mut dyn Host) {
    host.register_model::<Cuboid>();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cuboid {
    x: f64,
    y: f64,
    z: f64,
}

impl Model for Cuboid {
    fn from_context(ctx: &dyn Context) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let x: f64 = ctx
            .get_argument("x")
            .map(|arg| arg.parse().unwrap())
            .unwrap_or(3.0);
        let y: f64 = ctx
            .get_argument("y")
            .map(|arg| arg.parse().unwrap())
            .unwrap_or(2.0);
        let z: f64 = ctx
            .get_argument("z")
            .map(|arg| arg.parse().unwrap())
            .unwrap_or(1.0);

        Ok(Cuboid { x, y, z })
    }

    fn shape(&self) -> fj::Shape {
        let Cuboid { x, y, z } = *self;

        let rectangle = fj::Sketch::from_points(vec![
            [-x / 2., -y / 2.],
            [x / 2., -y / 2.],
            [x / 2., y / 2.],
            [-x / 2., y / 2.],
        ])
        .with_color([100, 255, 0, 200]);

        let cuboid = fj::Sweep::from_path(rectangle.into(), [0., 0., z]);

        cuboid.into()
    }
}
