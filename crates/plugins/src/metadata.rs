/// Information about a particular plugin that can be used by the host for
/// things like introspection and search.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
}

#[allow(missing_docs)]
impl PluginMetadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        PluginMetadata {
            name: name.into(),
            version: version.into(),
            short_description: None,
            description: None,
            homepage: None,
            repository: None,
            license: None,
        }
    }

    pub fn set_short_description(self, short_description: impl Into<String>) -> Self {
        let short_description = short_description.into();
        if short_description.is_empty() {
            return self;
        }

        PluginMetadata {
            short_description: Some(short_description),
            ..self
        }
    }

    pub fn set_description(self, description: impl Into<String>) -> Self {
        let description = description.into();
        if description.is_empty() {
            return self;
        }

        PluginMetadata {
            description: Some(description),
            ..self
        }
    }

    pub fn set_homepage(self, homepage: impl Into<String>) -> Self {
        let homepage = homepage.into();
        if homepage.is_empty() {
            return self;
        }

        PluginMetadata {
            homepage: Some(homepage),
            ..self
        }
    }

    pub fn set_repository(self, repository: impl Into<String>) -> Self {
        let repository = repository.into();
        if repository.is_empty() {
            return self;
        }

        PluginMetadata {
            repository: Some(repository),
            ..self
        }
    }

    pub fn set_license(self, license: impl Into<String>) -> Self {
        let license = license.into();
        if license.is_empty() {
            return self;
        }

        PluginMetadata {
            license: Some(license),
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelMetadata {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<ArgumentMetadata>,
}

impl ModelMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        ModelMetadata {
            name: name.into(),
            description: None,
            arguments: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_argument(mut self, arg: impl Into<ArgumentMetadata>) -> Self {
        self.arguments.push(arg.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentMetadata {
    /// The name used to refer to this argument.
    pub name: String,
    /// A short description of this argument that could be shown to the user
    /// in something like a tooltip.
    pub description: Option<String>,
    /// Something that could be used as a default if no value was provided.
    pub default_value: Option<String>,
}

impl ArgumentMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        ArgumentMetadata {
            name: name.into(),
            description: None,
            default_value: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
}

impl From<&str> for ArgumentMetadata {
    fn from(name: &str) -> Self {
        ArgumentMetadata::new(name)
    }
}
