use crate::app::AppControllerHandle;

pub struct EntryList{
    pub entries: Vec<String>,
}

impl EntryList{
    pub fn new() -> Self{
        Self{
            entries: Vec::new(),
        }
    }
}

impl super::Widget for EntryList{
    fn draw(&mut self, ctx: &super::WidgetContext, state: &super::AppState, controller: &mut AppControllerHandle){
        egui::CentralPanel::default().show(ctx.egui_ctx, |ui|{
            ui.heading("Entries");
            for (key, value) in state.logs.iter(){
                ui.collapsing("Entry", |ui|{
                    ui.label(key);
                   
                });
            }
        });
    }
}