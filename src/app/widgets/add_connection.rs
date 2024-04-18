use crate::app::AppControllerHandle;

pub struct AddConnectionWidget {
    name: String,
}

impl AddConnectionWidget {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
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

        ui.text_edit_singleline(&mut self.name);

        if ui.button("Connection").clicked() {
            let mut controller = controller.lock().unwrap();
            controller.new_mock_connection(self.name.clone());
        }
    }

    fn get_widget_options(&self) -> super::WidgetOptions {
        super::WidgetOptions {
            key: "add_connection".to_string(),
            title: "Add New Connection".to_string(),
        }
    }
}
