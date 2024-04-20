use std::collections::HashMap;

use egui::Style;
use egui_dock::TabViewer;

use super::{AppControllerHandle, AppState, AppWidgets, Widget, WidgetContext};

pub struct DockWindow {
    pub title: String,
    pub style: Option<Style>,
    pub current: String,
    pub windows: HashMap<String, AppWidgets>,
    pub controller_handle: AppControllerHandle,
}
impl DockWindow {
    pub fn new(title: String, controller: AppControllerHandle) -> Self {
        Self {
            title,
            style: None,
            current: "Logs".to_owned(),
            windows: HashMap::default(),
            controller_handle: controller,
        }
    }

    pub fn register_window(&mut self, widget: AppWidgets) {
        let key = widget.get_widget_options().key.clone();
        self.windows.insert(key, widget);
    }
}

impl TabViewer for DockWindow {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab) {
        let mut state = AppState::new();
        {
            state = self.controller_handle.lock().unwrap().state.clone();
        }
        let context = WidgetContext::new(ui);
        match self.windows.get_mut(tab.as_str()) {
            Some(widget) => widget.draw(context, &state, &mut self.controller_handle.clone()),
            None => {
                // set window to trasnparent

                ui.label("No widget");
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        self.windows
            .get(tab.as_str())
            .unwrap()
            .get_widget_options()
            .title
            .clone()
            .into()
    }
}
