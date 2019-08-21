use crate::DecentralizedIdentifer;

#[derive(Clone, Debug)]
pub struct DecentralizedIdentiferUrl {
    did: DecentralizedIdentifer,
    params: DecentralizedIdentiferParams,
    path: Option<String>,
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
            path: None,
        }
    }

    pub fn set_params(&mut self, params: DecentralizedIdentiferParams) {
        self.params = params;
    }

    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }
}

impl DecentralizedIdentiferParams {
    pub fn add_param(&mut self, param: DecentralizedIdentiferParam) {
        // TODO push functions should come from deref
        self.0.push(param)
    }
}

impl DecentralizedIdentiferParam {
    pub fn new(name: &str, value: Option<&str>) -> Self {
        DecentralizedIdentiferParam(name.to_string(), value.map(|x| x.to_string()))
    }
}
