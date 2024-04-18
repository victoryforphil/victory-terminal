use super::AppState;

mod add_connection;
pub use add_connection::*;

mod message_list;
use egui::Context;
use egui_dock::egui::Ui;
pub use message_list::*;

pub struct WidgetOptions {
    pub key: String,
    pub title: String,
}
pub struct WidgetContext<'a> {
    pub egui_ctx: Context,
    pub ui: &'a mut Ui,
}

impl<'a> WidgetContext<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        let ctx = ui.ctx().clone();
        Self {
            egui_ctx: ctx,
            ui: ui,
        }
    }
}

pub trait Widget {
    fn get_widget_options(&self) -> WidgetOptions;
    fn draw(
        &mut self,
        ctx: WidgetContext,
        state: &AppState,
        controller: &mut super::AppControllerHandle,
    );
}
