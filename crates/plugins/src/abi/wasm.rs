use crate::{ModelConstructor, PluginMetadata};
use wit_bindgen_rust::Handle;

wit_bindgen_rust::import!("../../wit-files/host.wit");
wit_bindgen_rust::export!("../../wit-files/guest.wit");

pub struct Guest;

impl guest::Guest for Guest {
    fn init() -> Result<Handle<Plugin>, guest::Error> {
        let mut host = Host::default();

        let metadata = unsafe { crate::abi::fornjot_plugin_init(&mut host)? };

        let Host { constructor } = host;
        let model_constructor = match constructor {
            Some(c) => c,
            None => {
                let err = crate::Error::from(
                    "No model registered. Did you forget to call the register_plugin!() macro?",
                );
                return Err(err.into());
            }
        };

        Ok(Handle::new(Plugin {
            model_constructor,
            metadata,
        }))
    }
}

pub struct Plugin {
    model_constructor: ModelConstructor,
    #[allow(dead_code)]
    metadata: PluginMetadata,
}

impl guest::Plugin for Plugin {
    fn load_model(&self, args: Vec<(String, String)>) -> Result<Handle<Model>, guest::Error> {
        let args = args.into_iter().collect();
        let ctx = crate::abi::Context(&args);

        let model = (self.model_constructor)(&ctx)?;

        Ok(Handle::new(Model(model)))
    }
}

#[derive(Default)]
struct Host {
    constructor: Option<ModelConstructor>,
}

impl crate::Host for Host {
    fn register_model_constructor(&mut self, constructor: ModelConstructor) {
        self.constructor = Some(constructor);
    }
}

pub struct Model(Box<dyn crate::Model>);

impl guest::Model for Model {
    fn shape(&self) -> guest::Shape {
        self.0.shape().into()
    }
}

impl From<crate::Error> for guest::Error {
    fn from(e: crate::Error) -> guest::Error {
        let message = e.to_string();
        let verbose_message = format!("{e:?}");

        let mut causes = Vec::new();
        let mut source = e.source();
        while let Some(s) = source {
            causes.push(s.to_string());
            source = s.source();
        }

        guest::Error {
            message,
            verbose_message,
            causes,
        }
    }
}

impl From<fj::Shape> for guest::Shape {
    fn from(s: fj::Shape) -> guest::Shape {
        match s {
            fj::Shape::Shape2d(s) => guest::Shape::Shape2d(s.into()),
            fj::Shape::Sweep(s) => guest::Shape::Sweep(s.into()),
            fj::Shape::Group(_) => unimplemented!(),
            fj::Shape::Transform(_) => unimplemented!(),
        }
    }
}

impl From<fj::Shape2d> for guest::Shape2d {
    fn from(s: fj::Shape2d) -> guest::Shape2d {
        match s {
            fj::Shape2d::Sketch(s) => guest::Shape2d::Sketch(s.into()),
            fj::Shape2d::Difference(_) => unimplemented!(),
        }
    }
}

impl From<fj::Sweep> for guest::Sweep {
    fn from(s: fj::Sweep) -> guest::Sweep {
        let shape = s.shape().clone().into();
        let [a, b, c] = s.path();

        guest::Sweep {
            shape,
            path: (a, b, c),
        }
    }
}

impl From<fj::Sketch> for guest::Sketch {
    fn from(s: fj::Sketch) -> guest::Sketch {
        let chain = s.chain().clone().into();
        let [r, g, b, a] = s.color();

        guest::Sketch {
            chain,
            color: (r, g, b, a),
        }
    }
}

impl From<fj::Chain> for guest::Chain {
    fn from(c: fj::Chain) -> guest::Chain {
        match c {
            fj::Chain::Circle(c) => guest::Chain::Circle(c.into()),
            fj::Chain::PolyChain(p) => guest::Chain::PolyChain(p.into()),
        }
    }
}

impl From<fj::PolyChain> for guest::PolyChain {
    fn from(p: fj::PolyChain) -> guest::PolyChain {
        let points = p.to_points().into_iter().map(|[x, y]| (x, y)).collect();

        guest::PolyChain { points }
    }
}

impl From<fj::Circle> for guest::Circle {
    fn from(c: fj::Circle) -> guest::Circle {
        guest::Circle { radius: c.radius() }
    }
}
