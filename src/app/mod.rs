mod controller;
mod dock_window;
mod state;
mod widgets;
use std::sync::{Arc, Mutex};

pub use controller::*;

use egui_dock::{DockArea, DockState, NodeIndex, Style};
pub use state::*;
pub use widgets::*;

use self::dock_window::DockWindow;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TerminalApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)] // This how you opt-out of serialization of a field
    dock_tree: DockState<String>,
    last_drock_tree: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    dock: DockWindow,
    #[serde(skip)] // This how you opt-out of serialization of a field
    controller: AppControllerHandle,
}

enum AppWidgets {
    AddConnection(AddConnectionWidget),
    MessageList(MessageList),
}

impl Widget for AppWidgets {
    fn draw(&mut self, ctx: WidgetContext, state: &AppState, controller: &mut AppControllerHandle) {
        match self {
            AppWidgets::AddConnection(widget) => widget.draw(ctx, state, controller),
            AppWidgets::MessageList(widget) => widget.draw(ctx, state, controller),
        }
    }

    fn get_widget_options(&self) -> WidgetOptions {
        match self {
            AppWidgets::AddConnection(widget) => widget.get_widget_options(),
            AppWidgets::MessageList(widget) => widget.get_widget_options(),
        }
    }
}

impl Default for TerminalApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![]);

        let controller = AppController::new();
        let controller_handle = Arc::new(Mutex::new(controller.clone()));
        let mut dock_window = DockWindow::new("Idk".to_string(), controller_handle.clone());

        Self::add_dock_widget(
            &mut tree,
            &mut dock_window,
            AppWidgets::AddConnection(AddConnectionWidget::new()),
        );
        Self::add_dock_widget_right(
            &mut tree,
            &mut dock_window,
            AppWidgets::MessageList(MessageList::new()),
        );

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            dock_tree: tree,
            dock: dock_window,
            last_drock_tree: "".to_owned(),
            controller: controller_handle,
        }
    }
}

impl TerminalApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        Default::default()
    }
    fn add_dock_widget(
        dock_tree: &mut DockState<String>,
        window: &mut DockWindow,
        widget: AppWidgets,
    ) {
        let key = widget.get_widget_options().key.clone();
        window.register_window(widget);

        dock_tree.push_to_focused_leaf(key.clone());
    }

    fn add_dock_widget_right(
        dock_tree: &mut DockState<String>,
        window: &mut DockWindow,
        widget: AppWidgets,
    ) {
        let key = widget.get_widget_options().key.clone();
        window.register_window(widget);

        dock_tree
            .main_surface_mut()
            .split_right(NodeIndex::root(), 0.5, vec![key.clone()]);
    }
}

impl eframe::App for TerminalApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.dock_tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.dock);
    }
}
