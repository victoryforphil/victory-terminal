use std::{collections::HashMap, hash::Hash};

use crate::{Connection, TerminalMessage};

#[derive(Clone, Debug)]
pub struct AppState{

    pub messages: HashMap<String, Vec<TerminalMessage>>,
}

impl AppState{
    pub fn new() -> Self{
        Self{
            messages: HashMap::new(),
        }
    }

    pub fn append_message(&mut self, connection_name: String, messages: Vec<TerminalMessage>){
        if let Some(connection_messages) = self.messages.get_mut(&connection_name){
            connection_messages.extend(messages);
        }else{
            self.messages.insert(connection_name, messages);
        }
    }
}