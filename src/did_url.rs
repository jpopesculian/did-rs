use crate::DecentralizedIdentifer;
use core::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferUrl {
    did: DecentralizedIdentifer,
    params: DecentralizedIdentiferParams,
    path: DecentralizedIdentiferPath,
}

#[derive(Clone, Debug, Default)]
pub struct DecentralizedIdentiferPath {
    path: Option<String>,
    params: DecentralizedIdentiferParams,
    fragment: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct DecentralizedIdentiferParams(Vec<DecentralizedIdentiferParam>);

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferParam(String, Option<String>);

impl DecentralizedIdentiferUrl {
    pub fn new(did: DecentralizedIdentifer) -> Self {
        DecentralizedIdentiferUrl {
            did,
            params: DecentralizedIdentiferParams::default(),
            path: DecentralizedIdentiferPath::default(),
        }
    }

    pub fn set_params(&mut self, params: DecentralizedIdentiferParams) {
        self.params = params;
    }

    pub fn set_path(&mut self, path: DecentralizedIdentiferPath) {
        self.path = path;
    }
}

impl DecentralizedIdentiferPath {
    pub fn new() -> Self {
        DecentralizedIdentiferPath {
            ..Default::default()
        }
    }

    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    pub fn set_params(&mut self, params: DecentralizedIdentiferParams) {
        self.params = params;
    }

    pub fn set_fragment(&mut self, fragment: Option<String>) {
        self.fragment = fragment;
    }
}

impl Deref for DecentralizedIdentiferParams {
    type Target = Vec<DecentralizedIdentiferParam>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DecentralizedIdentiferParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DecentralizedIdentiferParams {
    pub fn new() -> Self {
        DecentralizedIdentiferParams::default()
    }
}

impl DecentralizedIdentiferParam {
    pub fn new(name: &str, value: Option<&str>) -> Self {
        DecentralizedIdentiferParam(name.to_string(), value.map(|x| x.to_string()))
    }
}
