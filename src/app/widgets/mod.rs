use super::AppState;

mod add_entry;
pub use add_entry::*;

mod entry_list;
pub use entry_list::*;

pub struct WidgetContext<'a>{
    pub egui_ctx: &'a egui::Context,
}

impl<'a> WidgetContext<'a>{
    pub fn new(egui_ctx: &'a egui::Context) -> Self{
        Self{
            egui_ctx: egui_ctx,
        }
    }
}

pub trait Widget{
    fn draw(&mut self, ctx: &WidgetContext, state: &AppState, controller: &mut super::AppControllerHandle);
}