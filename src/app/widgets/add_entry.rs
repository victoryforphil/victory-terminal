use crate::app::AppControllerHandle;

pub struct AddEnryWidget{
    key: String,
    value: String,
}

impl AddEnryWidget{
    pub fn new() -> Self{
        Self{
            key: "".to_string(),
            value: "".to_string(),
        }
    }
}

impl super::Widget for AddEnryWidget{
    fn draw(&mut self, ctx: &super::WidgetContext, state: &super::AppState, controller: &mut AppControllerHandle){
        let ctx = ctx.egui_ctx;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Add Entry");
           
            ui.text_edit_singleline(&mut self.key);

           
            ui.text_edit_singleline(&mut self.value);

            if ui.button("Add").clicked(){
                let mut controller = controller.lock().unwrap();
                controller.new_entry(self.key.clone(), vec![self.value.clone()]);
            }
        });
    }
}