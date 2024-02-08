mod controller;
mod state;
mod widgets;
use std::sync::{Arc, Mutex};

pub use controller::*;
pub use state::*;
pub use widgets::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TerminalApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)] // This how you opt-out of serialization of a field
    widgets: Vec<AppWidgets>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    controller: AppControllerHandle,
}

enum AppWidgets{
    AddEntry(AddEnryWidget),
    EntryList(EntryList),
}

impl Widget for AppWidgets{
    fn draw(&mut self, ctx: &WidgetContext, state: &AppState, controller: &mut AppControllerHandle){
        match self{
            AppWidgets::AddEntry(widget) => widget.draw(ctx, state, controller),
            AppWidgets::EntryList(widget) => widget.draw(ctx, state, controller),
        }
    }
}

impl Default for TerminalApp {
    fn default() -> Self {
        let mut widgets = Vec::new();
        widgets.push(AppWidgets::AddEntry(AddEnryWidget::new()));
        widgets.push(AppWidgets::EntryList(EntryList::new()));
        let controller = AppController::new();
        let controller_handle = Arc::new(Mutex::new(controller.clone()));
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            widgets: widgets,
            controller: controller_handle,
        }
    }
}

impl TerminalApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        
        Default::default()
    }
}

impl eframe::App for TerminalApp {
 

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = AppState::new();
        {
            state = self.controller.lock().unwrap().state.clone();
        }
        for widget in &mut self.widgets {
            // Call draw on the widget trait object
            widget.draw(&WidgetContext::new(ctx), &state, &mut self.controller.clone());
        }
    }
}

