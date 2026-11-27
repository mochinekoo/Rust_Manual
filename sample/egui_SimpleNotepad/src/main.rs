use std::error::Error;
use std::path::PathBuf;
use eframe::egui;
use egui::{Color32, Response};
use rfd::{MessageButtons, MessageDialogResult, MessageLevel};

fn main() -> eframe::Result<()> {
    return eframe::run_native(
        "シンプルメモ帳",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            return Ok(Box::new(MyApp::default()))
        }),
    )
}
#[derive(Default)]
struct MyApp {
    text: String,
    filePath: PathBuf,
    edited: bool
}

impl MyApp {
    pub const COLOR_WHITE: Color32 = Color32::from_rgb(240, 240, 240);

    pub fn appErrorDialog(error: &mut dyn Error) {
        rfd::MessageDialog::new()
            .set_title("エラー")
            .set_description(format!("エラーが発生しました：{}", error))
            .set_level(rfd::MessageLevel::Error)
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
    }

    pub fn save(&mut self, ui: &mut egui::Ui) {
        if self.filePath.is_empty() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("テキストファイル", &["txt"])
                .add_filter("全てのファイル", &["*"])
                .save_file() {
                // 保存の処理
                self.filePath = path;
            }
        }

        match std::fs::write(self.filePath.to_string_lossy().into_owned(), self.text.to_string())  {
            Ok(_) => {
                self.edited = false;
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Title(self.filePath.to_string_lossy().into_owned()));
            }
            Err(e) => {
                rfd::MessageDialog::new()
                    .set_title("エラー")
                    .set_description(format!("エラーが発生しました：{}", e))
                    .set_level(rfd::MessageLevel::Error)
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
            }
        }
    }
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
                            if (self.edited) {
                                let dialog = rfd::MessageDialog::new()
                                    .set_title("保存")
                                    .set_description("保存されていない変更があります。保存しますか？")
                                    .set_level(MessageLevel::Info)
                                    .set_buttons(MessageButtons::YesNo)
                                    .show();
                                match dialog {
                                    MessageDialogResult::Yes => {
                                        Self::save(self, ui);
                                        self.text.clear();
                                    }
                                    MessageDialogResult::No => {
                                        self.text.clear();
                                    }
                                    _ => {}
                                }
                            }
                            else {
                                self.text.clear();
                            }
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Title("メモ帳".to_string()));
                        }
                        if (ui.button("開く").clicked()) {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("テキストファイル", &["txt"])
                                .add_filter("全てのファイル", &["*"])
                                .pick_file()  {
                                // println!("{:?}", path)
                                match std::fs::read_to_string(&path) {
                                    Ok(text) => {
                                        self.text = text;
                                        self.filePath = path;
                                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Title(self.filePath.to_string_lossy().into_owned()));
                                    },
                                    Err(mut e) => {
                                        Self::appErrorDialog(&mut e);
                                    }
                                }
                            }
                        }
                        if (ui.button("上書き保存").clicked()) {
                            Self::save(self, ui);
                        }
                        if (ui.button("終了").clicked()) {
                            if (self.edited) {
                                let dialog = rfd::MessageDialog::new()
                                    .set_title("保存")
                                    .set_description("保存されていない変更があります。保存しますか？")
                                    .set_level(MessageLevel::Info)
                                    .set_buttons(MessageButtons::YesNo)
                                    .show();
                                match dialog {
                                    MessageDialogResult::Yes => {
                                        Self::save(self, ui);
                                    }
                                    MessageDialogResult::No => {
                                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                    _ => {}
                                }
                            }
                            else {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });

                    ui.menu_button("ヘルプ", |ui| {
                        if (ui.button("このアプリについて").clicked()) {
                            println!("aa");
                        }
                    });
                });

                let textEdit: Response = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                );
                if (textEdit.changed()) {
                    //println!("changed!");
                    self.edited = true;
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Title(self.filePath.to_string_lossy().into_owned() + "(*)"));
                }
            });
    }
}