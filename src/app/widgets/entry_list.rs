use std::time::UNIX_EPOCH;

use crate::app::AppControllerHandle;
use chrono::prelude::*;
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
    fn draw(&mut self, ctx: super::WidgetContext, state: &super::AppState, controller: &mut AppControllerHandle){
        let ui = ctx.ui;
        ui.heading("Entries");
        for (key, value) in state.messages.iter(){
            ui.collapsing(key, |ui|{
               let mut messages = "".to_string();
                for message in value.iter(){
                 
                     messages.push_str(&format!("{}: {}\n",message.timestamp.format("%H:%M:%S"), message.as_string()));
                }
                // Reverse the order of the messages
                messages = messages.lines().rev().collect::<Vec<&str>>().join("\n");
                ui.style_mut().spacing.text_edit_width = ui.available_width();
          
                ui.code_editor(&mut messages);
                
            });
        }

        let mut controller = controller.lock().unwrap();
        controller.read_messages();

        // Request redraw at 10hz
        ctx.egui_ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
    fn get_widget_options(&self) -> super::WidgetOptions {
        super::WidgetOptions { key: "entry_list".to_string(), title: "All Entries".to_string() }
    }
}