use eframe::NativeOptions;
use egui::RichText;
use egui::TextBuffer;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

pub fn build_options() -> NativeOptions {
    const WINDOW_HEIGHT: f32 = 600.0;
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WINDOW_WIDTH: f32 = ASPECT_RATIO * WINDOW_HEIGHT;

    return eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };
}

pub fn run(options: NativeOptions) {
    eframe::run_simple_native("exp", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let path = "/tmp";
            env::set_current_dir(path).expect("Failed to set current directory");
            Graphics::set_heading(ui, path.to_owned());

            egui::ScrollArea::vertical().show(ui, |ui| {
                let directory = fs::read_dir(path).unwrap();

                println!("{:#?}", directory);

                for entry in directory {
                    match entry {
                        Ok(contents) => {
                            if contents.file_type().unwrap().is_dir() {
                                Graphics::label_entry(
                                    ui,
                                    contents.file_name(),
                                    EntryType::DIRECTORY,
                                );
                            }

                            if contents.file_type().unwrap().is_file() {
                                Graphics::label_entry(ui, contents.file_name(), EntryType::FILE)
                            }

                            if contents.file_type().unwrap().is_symlink() {
                                Graphics::label_entry(ui, contents.file_name(), EntryType::SYM_LINK)
                            }
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            process::exit(1);
                        }
                    }
                }
            });
        });
    })
    .unwrap_or_else(|err| {
        eprintln!("Error creating egui context:\n{err}");
        process::exit(1);
    })
}

pub enum EntryType {
    FILE,
    DIRECTORY,
    SYM_LINK,
}

pub struct Graphics;

impl Graphics {
    pub fn new() -> Self {
        return Self;
    }
    pub fn set_heading(ui: &mut egui::Ui, new_heading: String) {
        ui.heading(new_heading);
    }

    pub fn label_entry(ui: &mut egui::Ui, name: OsString, entry_type: EntryType) {
        let mut symbol;

        match entry_type {
            EntryType::FILE => {
                symbol = "";
            }

            EntryType::DIRECTORY => {
                symbol = "\u{1F4C1}";
            }

            EntryType::SYM_LINK => {
                symbol = "\u{1F517}";
            }
        }
        ui.label(RichText::new(format!(
            "{} {}",
            symbol,
            name.into_string().unwrap()
        )));
    }
}
