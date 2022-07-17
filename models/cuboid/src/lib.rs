use fj_plugins::{Context, ContextExt, HostExt, Model, ModelFromContext, PluginMetadata};

// TODO: replace this with a custom attribute.
fj_plugins::register_plugin!(|host| {
    let _span = tracing::info_span!("init").entered();

    host.register_model::<Cuboid>();
    tracing::info!("Registered cuboid");

    Ok(
        PluginMetadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .set_short_description(env!("CARGO_PKG_DESCRIPTION"))
            .set_repository(env!("CARGO_PKG_REPOSITORY"))
            .set_homepage(env!("CARGO_PKG_HOMEPAGE"))
            .set_license(env!("CARGO_PKG_LICENSE"))
            .set_description(include_str!("../README.md")),
    )
});

#[derive(Debug, Clone, PartialEq)]
pub struct Cuboid {
    x: f64,
    y: f64,
    z: f64,
}

impl ModelFromContext for Cuboid {
    fn from_context(ctx: &dyn Context) -> Result<Self, fj_plugins::Error>
    where
        Self: Sized,
    {
        let x: f64 = ctx.parse_optional_argument("x")?.unwrap_or(3.0);
        let y: f64 = ctx.parse_optional_argument("y")?.unwrap_or(2.0);
        let z: f64 = ctx.parse_optional_argument("z")?.unwrap_or(1.0);
        tracing::debug!(x, y, z, "Creating a cuboid model");

        Ok(Cuboid { x, y, z })
    }
}

impl Model for Cuboid {
    #[tracing::instrument]
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
