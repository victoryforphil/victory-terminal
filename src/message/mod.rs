use std::time::Instant;

use chrono::{DateTime, Local, Timelike};

#[derive(Debug, Clone)]
pub struct TerminalMessage{
    pub timestamp: DateTime<Local>,
    pub data: Vec<u8>,
}

impl TerminalMessage{
    pub fn new(data: Vec<u8>) -> Self{
        Self{
            timestamp: Local::now(),
            data,
        }
    }

    pub fn as_string(&self) -> String{
        String::from_utf8_lossy(&self.data).to_string()
    }

    pub fn from_string(data: String) -> Self{
        Self{
            timestamp: Local::now(),
            data: data.into_bytes(),
        }
    }

    pub fn as_number(&self) -> f32{
        String::from_utf8_lossy(&self.data).parse().unwrap()
    }
    
}