use eframe::egui;

pub fn render_ui(app: &mut crate::app::MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(format!("{} {} {}", app.nombre1, app.operation, app.nombre2));
        ui.label(format!("Résultat : {}", app.resultat));

        // Layout horizontal pour placer les boutons numériques et opérations côte à côte
        ui.horizontal(|ui| {
            // Partie gauche: Boutons numériques
            ui.vertical(|ui| {
                render_numeric_buttons(ui, app);
                ui.horizontal(|ui| {
                    // Taille uniforme pour les boutons d'action
                    if ui.add_sized([94.0, 50.0], egui::Button::new("Calculer")).clicked() {
                        app.calculate_result();
                    }
                    if ui.add_sized([94.0, 50.0], egui::Button::new("C")).clicked() {
                        app.clean();
                    }
                });
            });
            // Partie droite: Boutons d'opération
            ui.vertical(|ui| {
                render_operations(ui, app);
            });
        });
    });
}

fn render_operations(ui: &mut egui::Ui, app: &mut crate::app::MyApp) {
    ui.vertical(|ui| {
        // Taille uniforme pour les boutons d'opération
        for op in ["+", "-", "x", "/", "^"].iter() {
            if ui.add_sized([60.0, 50.0], egui::Button::new(*op)).clicked() {
                app.operation = op.to_string();
            }
        }
    });
}

fn render_numeric_buttons(ui: &mut egui::Ui, app: &mut crate::app::MyApp) {
    // Disposition améliorée pour le pavé numérique
    for row in [["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"], ["", "0", ""]].iter() {
        ui.horizontal(|ui| {
            for &num in row.iter() {
                if !num.is_empty() {
                    if ui.add_sized([60.0, 50.0], egui::Button::new(num)).clicked() {
                        if app.operation.is_empty() {
                            app.nombre1.push_str(num);
                        } else {
                            app.nombre2.push_str(num);
                        }
                    }
                } else {
                    // Espace vide pour la case sans bouton
                    ui.add_sized([60.0, 50.0], egui::Label::new(""));
                }
            }
        });
    }
}