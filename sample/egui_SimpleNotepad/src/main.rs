use eframe::egui;
use egui::Color32;

fn main() -> eframe::Result<()> {
    return eframe::run_native(
        "あぷり",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp {}))),
    )
}

struct MyApp {

}

impl MyApp {
    pub const COLOR_WHITE: Color32 = Color32::from_rgb(240, 240, 240);
}

impl eframe::App for MyApp {

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame, ) {
        ui.painter().rect_filled(ui.available_rect_before_wrap(), 0.0, Self::COLOR_WHITE);

        egui::Frame::new()
            .fill(Self::COLOR_WHITE)
            .show(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("ファイル", |ui| {
                        if (ui.button("新規").clicked()) {
                            println!("aa")
                        }
                        if (ui.button("開く").clicked()) {
                            println!("aa")
                        }
                        if (ui.button("上書き保存").clicked()) {
                            println!("aa")
                        }
                        if (ui.button("終了").clicked()) {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("ヘルプ", |ui| {
                        if (ui.button("このアプリについて").clicked()) {
                            println!("aa");
                        }
                    });
                });
            });
    }
}