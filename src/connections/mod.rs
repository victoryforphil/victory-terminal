use self::options::ConnectionOptions;


mod options;
pub trait Connection{
    fn get_options() -> ConnectionOptions;
    fn construct() -> Self;
    fn connect(&mut self);
    fn read() -> Vec<String>;
}