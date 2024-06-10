use eframe::NativeOptions;
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
            ui.heading("exp file Explorer");
        });
    })
    .unwrap_or_else(|err| {
        eprintln!("Error creating egui context:\n{err}");
        process::exit(1);
    })
}
