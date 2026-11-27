use std::error::Error;
use std::path::PathBuf;
use eframe::egui;
use egui::{Color32, Response};
use rfd::{MessageButtons, MessageDialogResult, MessageLevel};

fn main() -> eframe::Result<()> {
    return eframe::run_native(
        "あぷり",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp::default()))),
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
                            println!("aa");
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
                        if (ui.button("上書き保存").clicked()) {
                            println!("aa")
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
                                        // 上書き保存
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