use std::sync::Arc;

use crate::{Error, Key, Path, Resolver, Result};

wit_bindgen_wasmtime::export!("../../wit/ephemeral/spin-config.wit");
pub use spin_config::add_to_linker;

/// A component configuration interface implementation.
pub struct ComponentConfig {
    component_root: Path,
    resolver: Arc<Resolver>,
}

impl ComponentConfig {
    pub fn new(component_id: impl Into<String>, resolver: Arc<Resolver>) -> Result<Self> {
        let component_root = Path::new(component_id)?;
        Ok(Self {
            component_root,
            resolver,
        })
    }
}

impl<'a> spin_config::SpinConfig for ComponentConfig {
    fn get_config(
        &mut self,
        key: &str,
    ) -> std::result::Result<std::string::String, spin_config::Error> {
        let key = Key::new(key)?;
        let path = &self.component_root + key;
        Ok(dbg!(self.resolver.resolve(&path))?)
    }
}

impl From<Error> for spin_config::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidKey(msg) => Self::InvalidKey(msg),
            Error::InvalidSchema(msg) => Self::InvalidSchema(msg),
            Error::Provider(msg) => Self::Provider(msg.to_string()),
            other => Self::Other(format!("{:?}", other)),
        }
    }
}
