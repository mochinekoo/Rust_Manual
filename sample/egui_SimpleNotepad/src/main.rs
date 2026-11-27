use eframe::egui;

fn main() -> eframe::Result<()> {
    return eframe::run_native(
        "あぷり",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp))),
    )
}

struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame, ) {
    }
}