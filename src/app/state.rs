use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AppState{
    pub logs: HashMap<String, Vec<String>>,
}

impl AppState{
    pub fn new() -> Self{
        Self{
            logs: HashMap::new(),
        }
    }
}