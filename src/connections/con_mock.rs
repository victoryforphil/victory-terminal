use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tracing::{debug, info};

use crate::{Connection, TerminalMessage};

use super::options::{ConnectionOptionBuilder, ConnectionOptionType, ConnectionOptions};

#[derive(Debug, Clone)]
pub struct DataGenerator {
    pub counter: u32,
    pub date_string: String,

}


impl DataGenerator {
    pub fn new() -> Self {
        Self {
            counter: 0,
            date_string: String::new()
        }
    }

    pub fn generate_data(&mut self) -> TerminalMessage {
        self.counter += 1;
        self.date_string = format!("Data {}", self.counter);
        TerminalMessage::from_string(self.date_string.clone())
    }
}

#[derive(Debug)]
pub struct MockConnection {
    pub data_gen: DataGenerator,
    pub data_channel_rx: Receiver<TerminalMessage>,
    pub data_channel_tx: Sender<TerminalMessage>,
    pub delay: Duration,
}

impl MockConnection {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            data_gen: DataGenerator::new(),
            data_channel_rx: rx,
            data_channel_tx: tx,
            delay: Duration::from_millis(100)

        }
    }

    fn thread_start(&mut self) {
        let tx = self.data_channel_tx.clone();
        let mut data_generator = self.data_gen.clone();
        let delay = self.delay.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(delay);
            let message = data_generator.generate_data();
            tx.send(message).unwrap();
            debug!("Data sent");
        });
    }
}

impl Connection for MockConnection {
    fn get_options(&self) -> ConnectionOptions {
        let mut options = ConnectionOptions::new();

        let raw_string_options = ConnectionOptionBuilder::new()
            .name("Delay (s)".to_string())
            .default(ConnectionOptionType::Float(0.1))
            .description("Delay (s)".to_string())
            .max(ConnectionOptionType::Float(5.0))
            .min( ConnectionOptionType::Float(0.01))
            .value(ConnectionOptionType::Float(0.1))
            .build();

        options
            .options
            .insert("delay".to_string(), raw_string_options);
        options
    }

    fn connect(&mut self, opts: &ConnectionOptions) {
        let delay = opts.options.get("delay").unwrap();
        info!("Delay: {:?}", delay.value);
         match delay.value{
            ConnectionOptionType::Float(f_val) => {
                self.delay = Duration::from_secs_f32(f_val)
            },
            _ => panic!("Not ")
        };
        self.thread_start();
    }

    fn read(&self) -> Vec<crate::TerminalMessage> {
        let mut messages = Vec::new();

        let mut recv = self.data_channel_rx.try_recv();
        while let Ok(msg) = recv {
            messages.push(msg);
            recv = self.data_channel_rx.try_recv();
        }
        messages
    }
}
