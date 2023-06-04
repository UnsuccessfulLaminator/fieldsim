use crate::bodies::*;
use nannou::geom::Vec2;
use nannou_egui::egui;



pub trait UiConstructor<T> {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool;
    fn get_value(&self) -> T;
    fn reset(&mut self);
}



fn labelled_drag_value(ui: &mut egui::Ui, label: &str, value: &mut f32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value));
    });
}



#[derive(Default)]
pub struct PointChargeConstructor {
    x: f32, y: f32,
    mass: f32, charge: f32
}

impl UiConstructor<Box<dyn Body>> for PointChargeConstructor {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool {
        labelled_drag_value(ui, "x:", &mut self.x);
        labelled_drag_value(ui, "y:", &mut self.y);
        labelled_drag_value(ui, "charge:", &mut self.charge);
        labelled_drag_value(ui, "mass:", &mut self.mass);
        
        ui.button("OK").clicked()
    }

    fn get_value(&self) -> Box<dyn Body> {
        Box::new(PointCharge {
            charge: self.charge,
            mass: self.mass,
            pos: Vec2::new(self.x, self.y),
            vel: Vec2::ZERO
        })
    }

    fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.mass = 1.;
        self.charge = 1.;
    }
}
