mod calculs;
mod ui;
mod app;

use app::MyApp;
use eframe::egui;

fn main() {
    // Options de configuration de la fenêtre
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 400.0]),
        ..Default::default()
    };

    // Lancement de l'application
    eframe::run_native(
        "Calculatrice",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
}