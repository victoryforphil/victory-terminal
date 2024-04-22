mod con_mock;
use std::fmt::Debug;

pub use con_mock::*;

use crate::TerminalMessage;


mod options;

pub use options::*;
pub trait Connection: Debug {
    fn get_options(&self) -> ConnectionOptions;
    fn connect(&mut self, opts: &ConnectionOptions);
    fn read(&self) -> Vec<TerminalMessage>;
}
