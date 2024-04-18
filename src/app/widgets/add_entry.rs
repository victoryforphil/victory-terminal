use crate::app::AppControllerHandle;

pub struct AddEnryWidget{
    name: String,
}

impl AddEnryWidget{
    pub fn new() -> Self{
        Self{
            name: "".to_string(),
        }
    }
}

impl super::Widget for AddEnryWidget{
    fn draw(&mut self, ctx: super::WidgetContext, state: &super::AppState, controller: &mut AppControllerHandle){
        let ui = ctx.ui;
        ui.label("Add Entry");
           
            ui.text_edit_singleline(&mut self.name);


            if ui.button("Add").clicked(){
                let mut controller = controller.lock().unwrap();
                controller.new_mock_connection(self.name.clone());
            }
    }

    fn get_widget_options(&self) -> super::WidgetOptions {
        super::WidgetOptions { key: "add_entry".to_string(), title: "Add New entry".to_string() }
    }
}