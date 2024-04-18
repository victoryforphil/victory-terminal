use crate::app::AppControllerHandle;
use chrono::prelude::*;
pub struct MessageList {
    pub entries: Vec<String>,
}

impl MessageList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl super::Widget for MessageList {
    fn draw(
        &mut self,
        ctx: super::WidgetContext,
        state: &super::AppState,
        controller: &mut AppControllerHandle,
    ) {
        let ui = ctx.ui;
        ui.heading("Messages");
        for (key, value) in state.messages.iter() {
            ui.label(key);
            let mut messages = "".to_string();
            for message in value.iter() {
                messages.push_str(&format!(
                    "{}: {}\n",
                    message.timestamp.format("%H:%M:%S"),
                    message.as_string()
                ));
            }
            // Reverse the order of the messages
            messages = messages.lines().rev().collect::<Vec<&str>>().join("\n");
            ui.style_mut().spacing.text_edit_width = ui.available_width();

            ui.code_editor(&mut messages);
            ui.separator();
        }

        let mut controller = controller.lock().unwrap();
        controller.read_messages();

        // Request redraw at 10hz
        ctx.egui_ctx
            .request_repaint_after(std::time::Duration::from_millis(100));
    }
    fn get_widget_options(&self) -> super::WidgetOptions {
        super::WidgetOptions {
            key: "message_list".to_string(),
            title: "Messages".to_string(),
        }
    }
}
