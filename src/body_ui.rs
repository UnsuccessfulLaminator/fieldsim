use crate::bodies::*;
use nannou_egui::egui;


pub trait UiConstructor<T> {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool;
    fn get_value(&self) -> T;
}



fn labelled_drag_value(ui: &mut egui::Ui, label: &str, value: &mut f32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value));
    });
}



impl UiConstructor<Box<dyn Body>> for PointCharge {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool {
        labelled_drag_value(ui, "x:", &mut self.pos[0]);
        labelled_drag_value(ui, "y:", &mut self.pos[1]);
        labelled_drag_value(ui, "charge:", &mut self.charge);
        
        ui.button("OK").clicked()
    }

    fn get_value(&self) -> Box<dyn Body> {
        Box::new(self.clone())
    }
}
