use eframe::egui;
use crate::calculs;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::ui::render_ui(self, ctx);
    }
}
#[derive(Default)]
pub struct MyApp {
    pub nombre1: String,
    pub nombre2: String,
    pub operation: String,
    pub resultat: String,
}

impl MyApp {
    pub fn calculate_result(&mut self) {
        let n1: f64 = self.nombre1.trim().parse().unwrap_or(0.0);
        let n2: f64 = self.nombre2.trim().parse().unwrap_or(0.0);
        self.resultat = match self.operation.trim() {
            "+" => calculs::addition(n1 as i32, n2 as i32).to_string(),
            "-" => calculs::soustraction(n1 as i32, n2 as i32).to_string(),
            "x" => calculs::multiplication(n1 as i32, n2 as i32).to_string(),
            "/" => match calculs::division(n1 as i32, n2 as i32) {
                Ok(res) => res.to_string(),
                Err(err) => err,
            },
            "^" => calculs::carre(n1 as i32).to_string(),
            _ => "Opération non reconnue".to_string(),
        };
    }

    pub fn clean(&mut self) {
        self.nombre1.clear();
        self.nombre2.clear();
        self.operation.clear();
        self.resultat.clear();
    }
}