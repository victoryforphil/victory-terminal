mod con_mock;
use std::fmt::Debug;

pub use con_mock::*;

use crate::TerminalMessage;

use self::options::ConnectionOptions;

mod options;

pub trait Connection: Debug {
    fn get_options(&self) -> ConnectionOptions;
    fn connect(&mut self);
    fn read(&self) -> Vec<TerminalMessage>;
}
