use std::fmt::Display;

use egui::ComboBox;

use crate::{app::AppControllerHandle, Connection, ConnectionOptionType, ConnectionOptions, MockConnection};

pub struct AddConnectionWidget {
    name: String,
    pub current_connection: EnabledConnections,
    pub current_config: Option<ConnectionOptions>,
}

impl AddConnectionWidget {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            current_connection: EnabledConnections::Mock,
            current_config: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnabledConnections {
    Mock,
}
impl Display for EnabledConnections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnabledConnections::Mock => write!(f, "Mock Connection"),
        }
    }
}

impl EnabledConnections {
    pub fn get_config(&self) -> ConnectionOptions {
        match self {
            EnabledConnections::Mock => MockConnection::new().get_options(),
        }
    }

    pub fn connect(&self, opts: &ConnectionOptions) {
        match self {
            EnabledConnections::Mock => MockConnection::new().connect(opts),
        }
    }
}

impl super::Widget for AddConnectionWidget {
    fn draw(
        &mut self,
        ctx: super::WidgetContext,
        _state: &super::AppState,
        controller: &mut AppControllerHandle,
    ) {
        let ui = ctx.ui;
        ui.label("Add Connection");

        ComboBox::from_label("Connection Type")
            .selected_text(format!("{}", self.current_connection))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.current_connection,
                    EnabledConnections::Mock,
                    format!("Mock Connection"),
                );
            });

        let connection = &mut self.current_connection;
        self.current_config
        .get_or_insert_with(|| connection.get_config());

        let config = self.current_config.as_mut().unwrap();
        for (key, option) in config.options.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(format!("{}: ", key));
                match &mut option.value {
                    ConnectionOptionType::Float(num) => {
                        // Slider
                       
                        let max = match option.max {
                            Some(ConnectionOptionType::Float(max_v)) => max_v ,
                            _ => 100.0,
                        };
                        let min = match option.min {
                            Some(ConnectionOptionType::Float(min_v)) => min_v ,
                            _ => 0.1,
                        };
                        ui.add(
                            egui::Slider::new( num, min..=max)
                                .text(key)
                                .clamp_to_range(true)
                             
                        );
                    },
                    _ => {
                       ui.label("Not implemented");
                    }
                   
                }
            });
        }

        if ui.button("Connect").clicked() {
           let mut controller = controller.lock().unwrap();
           controller.new_mock_connection("new con".to_string(), &self.current_config.as_ref().unwrap());

        }
    }

    fn get_widget_options(&self) -> super::WidgetOptions {
        super::WidgetOptions {
            key: "add_connection".to_string(),
            title: "Add New Connection".to_string(),
        }
    }
}
