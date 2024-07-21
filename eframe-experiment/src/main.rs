use eframe::{egui, run_native, NativeOptions, App, Frame};

struct MyApp {
    selected_tab: usize,
    input_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            input_text: String::new(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Tab 1").clicked() {
                    self.selected_tab = 0;
                }
                if ui.button("Tab 2").clicked() {
                    self.selected_tab = 1;
                }
                if ui.button("Tab 3").clicked() {
                    self.selected_tab = 2;
                }
            });

            ui.separator();

            match self.selected_tab {
                0 => {
                    ui.label("This is the content of Tab 1.");
                    ui.text_edit_singleline(&mut self.input_text);
                }
                1 => {
                    ui.label("This is the content of Tab 2.");
                    ui.text_edit_singleline(&mut self.input_text);
                }
                2 => {
                    ui.label("This is the content of Tab 3.");
                    ui.text_edit_singleline(&mut self.input_text);
                }
                _ => {}
            }
        });
    }

    //fn setup(&mut self, _ctx: &egui::Context, _frame: &mut Frame, _storage: Option<&dyn eframe::Storage>) {
    //    // ここにアプリケーションのセットアップコードを追加できます
    //}
}

fn main() {
    let app = MyApp::default();
    let native_options = NativeOptions::default();
    run_native(
        "Tabbed App Example",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
