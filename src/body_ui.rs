use crate::bodies::*;
use nannou::geom::Vec2;
use nannou_egui::egui;
use std::ops::RangeInclusive;



pub trait UiConstructor<T> {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool;
    fn get_value(&self) -> T;
    fn reset(&mut self);
}



fn labelled_widget(ui: &mut egui::Ui, label: &str, widget: impl egui::Widget) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(widget);
    });
}

fn labelled_drag_value(ui: &mut egui::Ui, label: &str, value: &mut f32) {
    labelled_widget(ui, label, egui::DragValue::new(value));
}

fn labelled_slider(
    ui: &mut egui::Ui, label: &str, value: &mut f32, range: RangeInclusive<f32>
) {
    labelled_widget(ui, label, egui::Slider::new(value, range));
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
        
        if self.mass < 0.1 { self.mass = 0.1; }
        
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



#[derive(Default)]
pub struct DipoleConstructor {
    x: f32, y: f32, angle_deg: f32,
    dipole: f32, mass: f32
}

impl UiConstructor<Box<dyn Body>> for DipoleConstructor {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool {
        labelled_drag_value(ui, "x:", &mut self.x);
        labelled_drag_value(ui, "y:", &mut self.y);
        labelled_slider(ui, "angle:", &mut self.angle_deg, 0.0..=360.0);
        labelled_drag_value(ui, "dipole:", &mut self.dipole);
        labelled_drag_value(ui, "mass:", &mut self.mass);

        if self.mass < 0.1 { self.mass = 0.1; }
        if self.dipole < 0. { self.dipole = 0.; }
        
        ui.button("OK").clicked()
    }

    fn get_value(&self) -> Box<dyn Body> {
        Box::new(Dipole::new(
            self.dipole, self.mass, self.angle_deg.to_radians(),
            Vec2::new(self.x, self.y), Vec2::ZERO
        ))
    }

    fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.angle_deg = 0.;
        self.dipole = 1.;
        self.mass = 1.;
    }
}



#[derive(Default)]
pub struct CircleChargeConstructor {
    x: f32, y: f32, radius: f32,
    charge: f32, mass: f32
}

impl UiConstructor<Box<dyn Body>> for CircleChargeConstructor {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool {
        labelled_drag_value(ui, "x:", &mut self.x);
        labelled_drag_value(ui, "y:", &mut self.y);
        labelled_drag_value(ui, "radius:", &mut self.radius);
        labelled_drag_value(ui, "charge:", &mut self.charge);
        labelled_drag_value(ui, "mass:", &mut self.mass);

        if self.mass < 0.1 { self.mass = 0.1; }
        if self.radius < 0.5 { self.radius = 0.5; }
        
        ui.button("OK").clicked()
    }

    fn get_value(&self) -> Box<dyn Body> {
        Box::new(CircleCharge {
            charge: self.charge,
            mass: self.mass,
            radius: self.radius,
            pos: Vec2::new(self.x, self.y),
            vel: Vec2::ZERO
        })
    }

    fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.radius = 5.;
        self.charge = 1.;
        self.mass = 1.;
    }
}



#[derive(Default)]
pub struct GlobalFieldConstructor {
    e_x: f32, e_y: f32
}

impl UiConstructor<Box<dyn Body>> for GlobalFieldConstructor {
    fn make_ui(&mut self, ui: &mut egui::Ui) -> bool {
        labelled_drag_value(ui, "Ex:", &mut self.e_x);
        labelled_drag_value(ui, "Ey:", &mut self.e_y);
        
        ui.button("OK").clicked()
    }

    fn get_value(&self) -> Box<dyn Body> {
        Box::new(GlobalField {
            field: Vec2::new(self.e_x, self.e_y)
        })
    }

    fn reset(&mut self) {
        self.e_x = 0.;
        self.e_y = 0.;
    }
}
