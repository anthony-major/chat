use eframe::{
    egui::{self, Context, ViewportCommand},
    Frame,
};
use tokio::sync::mpsc::error::TryRecvError;

use crate::{client::Client, message::Message};

pub struct App {
    client: Client,
    text_input: String,
    messages: Vec<Message>,
}

impl App {
    pub fn new(client: Client) -> Self {
        Self {
            client: client,
            text_input: String::new(),
            messages: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self.client.read().try_recv() {
            Ok(message) => {
                self.messages.push(message);
            }
            Err(TryRecvError::Disconnected) => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            _ => {}
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(ui.available_height() - 30.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for message in &self.messages {
                        ui.label(message.to_string());
                    }
                });

            ui.separator();

            let response = egui::TextEdit::singleline(&mut self.text_input)
                .desired_width(f32::INFINITY)
                .show(ui)
                .response;
            let text_input_submitted =
                response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

            if text_input_submitted {
                let message = Message::new(self.client.username().clone(), self.text_input.clone());
                self.client.send().blocking_send(message).unwrap();
                self.text_input.clear();
                response.request_focus();
            };
        });

        // This is to get our app to run in continuous mode, so new messages are seen immediately.
        ctx.request_repaint();
    }
}
