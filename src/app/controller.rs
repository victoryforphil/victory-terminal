use std::sync::{Arc, Mutex};

use super::AppState;
#[derive(Clone, Debug)]
pub struct AppController{
    pub state: AppState,
}

impl AppController{
    pub fn new() -> Self{
        Self{
            state: AppState::new(),
        }
    }

    pub fn new_entry(&mut self, key: String, value: Vec<String>){
        self.state.logs.insert(key, value);
    }
}

// Mutex Arc type for the app controller
pub type AppControllerHandle= Arc<Mutex<AppController>>;