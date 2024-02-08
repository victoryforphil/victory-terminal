use std::{collections::HashMap, default};
#[derive(Clone, PartialEq, Debug)]
pub struct SelectionData{
    pub options: Vec<String>,
    pub selected: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ConnectionOptionType{
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Selection(SelectionData),
}

impl Default for ConnectionOptionType{
    fn default() -> Self {
       ConnectionOptionType::String(String::default())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ConnectionOption{
    pub name: String,
    pub description: String,
    pub value: ConnectionOptionType,
    pub default: ConnectionOptionType,
    pub min: Option<ConnectionOptionType>, // only used for int and float
    pub max: Option<ConnectionOptionType>, // only used for int and float
}

impl Default for ConnectionOption{
    fn default() -> Self {
        Self { name: Default::default(), description: Default::default(), value: Default::default(), default: Default::default(), min: Default::default(), max: Default::default() }
    }
}

impl ConnectionOption{
    pub fn verify(self) -> Result<Self, String>
    {
        match &self.value{
            ConnectionOptionType::String(_) => todo!(),
            ConnectionOptionType::Int(int_value) => {
                if let Some(min) = &self.min{
                    if let ConnectionOptionType::Int(min_value) = min{
                        if int_value < min_value{
                            return Err(format!("Value {} is less than min {}", int_value, min_value));
                        }
                    }
                }
                if let Some(max) = &self.max{
                    if let ConnectionOptionType::Int(max_value) = max{
                        if int_value > max_value{
                            return Err(format!("Value {} is greater than max {}", int_value, max_value));
                        }
                    }
                }
                Ok(self)
            },
            ConnectionOptionType::Float(float_value) => {
                if let Some(min) = &self.min{
                    if let ConnectionOptionType::Float(min_value) = min{
                        if float_value < min_value{
                            return Err(format!("Value {} is less than min {}", float_value, min_value));
                        }
                    }
                }
                if let Some(max) = &self.max{
                    if let ConnectionOptionType::Float(max_value) = max{
                        if float_value > max_value{
                            return Err(format!("Value {} is greater than max {}", float_value, max_value));
                        }
                    }
                }
                Ok(self)
            },
            ConnectionOptionType::Bool(_) => Ok(self),
            ConnectionOptionType::Selection(_) => Ok(self),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionOptions{
    pub options: HashMap<String, ConnectionOption>,
}

impl ConnectionOptions{
    pub fn new() -> Self{
        Self{
            options: HashMap::new(),
        }
    }
}

pub struct ConnectionOptionBuilder{
    pub option: ConnectionOption
}

impl ConnectionOptionBuilder{
    pub fn new() -> Self{
        Self{
            option: ConnectionOption::default()
        }
    }

    pub fn name(mut self, name: String) -> Self{
        self.option.name = name;
        self
    }

    pub fn description(mut self, description: String) -> Self{
        self.option.description = description;
        self
    }

    pub fn value(mut self, value: ConnectionOptionType) -> Self{
        self.option.value = value;
        self
    }

    pub fn default(mut self, default: ConnectionOptionType) -> Self{
        self.option.default = default;
        self
    }

    pub fn min(mut self, min: ConnectionOptionType) -> Self{
        // Verify that the type is valid for min/max
        match min{
            ConnectionOptionType::Int(_) => {},
            ConnectionOptionType::Float(_) => {},
            _ => panic!("Invalid type for min/max")
        }
        self.option.min = Some(min);
        self
    }

    pub fn max(mut self, max: ConnectionOptionType) -> Self{
        // Verify that the type is valid for min/max
        match max{
            ConnectionOptionType::Int(_) => {},
            ConnectionOptionType::Float(_) => {},
            _ => panic!("Invalid type for min/max")
        }
        self.option.max = Some(max);
        self
    }

    pub fn build(self) -> ConnectionOption{
        self.option
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_option_builder_basic_int(){
        let option = ConnectionOptionBuilder::new()
            .name("Test Int".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::Int(5))
            .build();
        assert_eq!(option.name, "Test Int");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::Int(5));

    }

    #[test]
    fn test_option_builder_basic_float(){
        let option = ConnectionOptionBuilder::new()
            .name("Test Float".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::Float(5.5))
            .build();
        assert_eq!(option.name, "Test Float");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::Float(5.5));

    }


    #[test]
    fn test_option_builder_basic_bool(){
        let option = ConnectionOptionBuilder::new()
            .name("Test Bool".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::Bool(true))
            .build();
        assert_eq!(option.name, "Test Bool");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::Bool(true));

    }

    #[test]
    fn test_option_builder_basic_string(){
        let option = ConnectionOptionBuilder::new()
            .name("Test String".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::String("Test String".to_owned()))
            .build();
        assert_eq!(option.name, "Test String");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::String("Test String".to_owned()));

    }

    #[test]
    fn test_option_builder_basic_selection(){
        let option = ConnectionOptionBuilder::new()
            .name("Test Selection".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::Selection(SelectionData{options: vec!["Test1".to_owned(), "Test2".to_owned()], selected: 0}))
            .build();
        assert_eq!(option.name, "Test Selection");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::Selection(SelectionData{options: vec!["Test1".to_owned(), "Test2".to_owned()], selected: 0}));

    }

    #[test]
    fn test_option_builder_min_max_int(){
        let option = ConnectionOptionBuilder::new()
            .name("Test Int".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::Int(25))
            .min(ConnectionOptionType::Int(0))
            .max(ConnectionOptionType::Int(10))
            .build();
        assert_eq!(option.name, "Test Int");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::Int(25));
        assert_eq!(option.min, Some(ConnectionOptionType::Int(0)));
        assert_eq!(option.max, Some(ConnectionOptionType::Int(10)));

        assert_eq!(option.verify().is_ok(), false);
    }

    #[test]
    fn test_option_builder_min_max_float(){
        let option = ConnectionOptionBuilder::new()
            .name("Test Float".to_owned())
            .description("Test description".to_owned())
            .value(ConnectionOptionType::Float(25.0))
            .min(ConnectionOptionType::Float(0.0))
            .max(ConnectionOptionType::Float(10.0))
            .build();
        assert_eq!(option.name, "Test Float");
        assert_eq!(option.description, "Test description");
        assert_eq!(option.value, ConnectionOptionType::Float(25.0));
        assert_eq!(option.min, Some(ConnectionOptionType::Float(0.0)));
        assert_eq!(option.max, Some(ConnectionOptionType::Float(10.0)));

        assert_eq!(option.verify().is_ok(), false);
    }
}