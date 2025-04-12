use eframe::egui;

pub fn render_ui(app: &mut crate::MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(format!("{} {} {}", app.nombre1, app.operation, app.nombre2));
        ui.label(format!("Résultat : {}", app.resultat));

        render_operations(ui, app);
        render_numeric_buttons(ui, app);

        if ui.add_sized([168.0, 25.0], egui::Button::new("Calculer")).clicked() {
            app.calculate_result();
        }
        if ui.add_sized([168.0, 25.0], egui::Button::new("C")).clicked() {
            app.clean();
        }
    });
}

fn render_operations(ui: &mut egui::Ui, app: &mut crate::MyApp) {
    ui.horizontal(|ui| {
        for op in ["+", "-", "x", "/", "^"].iter() {
            if ui.add_sized([26.5, 26.5], egui::Button::new(*op)).clicked() {
                app.operation = op.to_string();
            }
        }
    });
}

fn render_numeric_buttons(ui: &mut egui::Ui, app: &mut crate::MyApp) {
    for row in [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"], ["0", "0", "0"]].iter() {
        ui.horizontal(|ui| {
            for &num in row.iter() {
                if ui.add_sized([50.0, 50.0], egui::Button::new(num)).clicked() {
                    if app.operation.is_empty() {
                        app.nombre1.push_str(num);
                    } else {
                        app.nombre2.push_str(num);
                    }
                }
            }
        });
    }
}