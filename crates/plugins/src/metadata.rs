/// Information about a particular plugin that can be used by the host for
/// things like introspection and search.
#[derive(Debug, Clone, PartialEq)]
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
            short_description: Some(short_description.into()),
            ..self
        }
    }

    pub fn set_description(self, description: impl Into<String>) -> Self {
        let description = description.into();
        if description.is_empty() {
            return self;
        }

        PluginMetadata {
            description: Some(description.into()),
            ..self
        }
    }

    pub fn set_homepage(self, homepage: impl Into<String>) -> Self {
        let homepage = homepage.into();
        if homepage.is_empty() {
            return self;
        }

        PluginMetadata {
            homepage: Some(homepage.into()),
            ..self
        }
    }

    pub fn set_repository(self, repository: impl Into<String>) -> Self {
        let repository = repository.into();
        if repository.is_empty() {
            return self;
        }

        PluginMetadata {
            repository: Some(repository.into()),
            ..self
        }
    }

    pub fn set_license(self, license: impl Into<String>) -> Self {
        let license = license.into();
        if license.is_empty() {
            return self;
        }

        PluginMetadata {
            license: Some(license.into()),
            ..self
        }
    }
}
