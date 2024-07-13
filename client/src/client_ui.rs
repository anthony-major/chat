use std::sync::Arc;

use tokio::sync::Mutex;

use eframe::{
    egui::{self, Context},
    Frame,
};

use crate::{client::Client, message::Message};

pub struct App {
    client: Arc<Mutex<Client>>,
    text_input: String,
}

impl App {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            text_input: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(ui.available_height() - 30.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    if let Ok(client) = self.client.try_lock() {
                        for message in client.messages() {
                            ui.label(message.to_string());
                        }
                    }
                });

            ui.separator();
            let response = egui::TextEdit::singleline(&mut self.text_input)
                .desired_width(f32::INFINITY)
                .show(ui)
                .response;
            if response.lost_focus() {
                let client = self.client.clone();
                let content = self.text_input.clone();
                let ctx_clone = ctx.clone();

                tokio::spawn(async move {
                    let message = Message::new(client.lock().await.username().clone(), content);
                    if let Err(_) = client.lock().await.send_message(message).await {
                        ctx_clone.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                self.text_input.clear();
                response.request_focus();
            };
        });
    }
}
