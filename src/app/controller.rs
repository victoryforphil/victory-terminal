use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tracing::{debug, info};

use crate::{Connection, ConnectionOptions, MockConnection};

use super::AppState;
#[derive(Clone, Debug)]
pub struct AppController {
    pub state: AppState,
    pub connections: HashMap<String, Arc<Mutex<dyn Connection>>>,
}

impl AppController {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            connections: HashMap::new(),
        }
    }

    pub fn new_mock_connection(&mut self, name: String, opts: &ConnectionOptions) {
        let mut connection = MockConnection::new();
        connection.connect(opts);
        info!("Mock connection {} created", name);
        self.connections
            .insert(name, Arc::new(Mutex::new(connection)));
    }
    pub fn read_messages(&mut self) {
        for (name, connection) in self.connections.iter_mut() {
            let connection = connection.lock().unwrap();
            let messages = connection.read();
            if messages.len() > 0 {
                debug!(
                    "Messages read from connection {}: {:?}",
                    name,
                    messages.len()
                );
            }
            self.state.append_message(name.clone(), messages);
        }
    }
}

// Mutex Arc type for the app controller
pub type AppControllerHandle = Arc<Mutex<AppController>>;
