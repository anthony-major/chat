use eframe::{
    egui::{self, Context},
    Frame,
};

pub struct App {
    messages: Vec<String>,
    text_input: String,
}

impl Default for App {
    fn default() -> Self {
        let lines: Vec<String> = (1..=100).map(|i| i.to_string()).collect();

        Self {
            messages: lines,
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
                    for line in &self.messages {
                        ui.label(line);
                    }
                });

            ui.separator();
            let response = ui.text_edit_singleline(&mut self.text_input);
            if response.lost_focus() {
                self.messages.push(self.text_input.clone());
                self.text_input.clear();
                response.request_focus();
            };
        });
    }
}
