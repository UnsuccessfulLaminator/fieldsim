mod bodies;
mod util;
mod body_ui;

use std::collections::HashMap;
use nannou::prelude::*;
use nannou::winit;
use nannou_egui::{egui, Egui};
use bodies::*;
use body_ui::*;



fn main() {
    nannou::app(model)
           .loop_mode(LoopMode::RefreshSync)
           .update(update)
           .run();
}

enum State {
    Simulating,
    ShowGui,
    AddIsopotential,
    AddBody(String)
}

struct Model {
    state: State,
    bodies: Vec<Box<dyn Body>>,
    isopotentials: Vec<Vec<Vec2>>,
    field_lines: Vec<Vec<Vec2>>,
    constructors: HashMap<String, Box<dyn UiConstructor<Box<dyn Body>>>>,
    selected_constructor: String,
    egui: Egui
}

fn model(app: &App) -> Model {
    let window_id = app.new_window()
                       .raw_event(raw_window_event)
                       .key_pressed(key_pressed)
                       .mouse_pressed(mouse_pressed)
                       .view(view)
                       .build()
                       .unwrap();
    
    let window = app.window(window_id).unwrap();
    let screen = app.window_rect();
    let mut model = Model {
        state: State::ShowGui,
        bodies: Vec::new(),
        isopotentials: Vec::new(),
        field_lines: Vec::new(),
        constructors: HashMap::new(),
        selected_constructor: String::new(),
        egui: Egui::from_window(&window)
    };
    
    model.constructors.insert(
        "Point charge".to_string(), Box::new(PointChargeConstructor::default())
    );

    model.constructors.insert(
        "Dipole".to_string(), Box::new(DipoleConstructor::default())
    );
    
    model.constructors.insert(
        "Circle charge".to_string(), Box::new(CircleChargeConstructor::default())
    );
    
    model.constructors.insert(
        "Global field".to_string(), Box::new(GlobalFieldConstructor::default())
    );

    model.selected_constructor = model.constructors.keys().nth(0).unwrap().clone();

    model.bodies.push(Box::new(LineCharge::new(
        Vec2::new(-100., 0.), Vec2::new(100., 0.), -100.
    )));

    model.bodies.push(Box::new(CircleCharge {
        charge: 50.,
        mass: 1.,
        radius: 20.,
        pos: Vec2::Y*50.,
        vel: Vec2::ZERO
    }));
    
    model.bodies.push(Box::new(CircleCharge {
        charge: 50.,
        mass: 1.,
        radius: 20.,
        pos: -Vec2::Y*50.,
        vel: Vec2::ZERO
    }));

    model
}

fn raw_window_event(_app: &App, model: &mut Model, event: &winit::event::WindowEvent) {
    if matches!(model.state, State::ShowGui | State::AddBody(_)) {
        model.egui.handle_raw_event(event);
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.state = match model.state {
                State::Simulating => State::ShowGui,
                State::ShowGui => State::Simulating,
                _ => { return; }
            }
        }
        _ => {}
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    let pos = Vec2::new(app.mouse.x, app.mouse.y);

    match button {
        MouseButton::Left => {
            if matches!(model.state, State::AddIsopotential) {
                let (mut points, is_loop) = util::isopotential_points(
                    &model.bodies, pos,
                    5e-3, 5., 1e-3, 1000
                );
                
                if is_loop { points.push(points[0]); }

                model.isopotentials.push(points);
                model.state = State::ShowGui;
            }
        }
        _ => {}
    }
}

fn make_ui(model: &mut Model) {
    let ctx = model.egui.begin_frame();

    egui::Window::new("Menu").show(&ctx, |ui| {
        if ui.button("Add isopotential").clicked() {
            model.state = State::AddIsopotential;
        }

        if ui.button("Draw field lines").clicked() {
            model.field_lines.clear();

            for isopotential in &model.isopotentials {
                let origins = util::divide_isopotential(&model.bodies, isopotential, 10.);
                
                for origin in origins {
                    let points = util::field_line_points(
                        &model.bodies, origin,
                        5e-3, 5., 1e-3, 1000
                    );

                    model.field_lines.push(points);
                }
            }
        }

        if ui.button("Clear lines").clicked() {
            model.isopotentials.clear();
            model.field_lines.clear();
        }
        
        ui.horizontal(|ui| {
            let selected = &mut model.selected_constructor;

            egui::ComboBox::from_label("")
                .selected_text(selected.clone())
                .show_ui(ui, |ui| {
                    for key in model.constructors.keys() {
                        ui.selectable_value(selected, key.to_string(), key.to_string());
                    }
                });

            if ui.button("Add").clicked() {
                model.state = State::AddBody(selected.clone());
                model.constructors.get_mut(selected).unwrap().reset();
            }
        });
    });
}

fn simulate(model: &mut Model, dt: f32) {
    for i in 0..model.bodies.len() {
        let pos = model.bodies[i].pos();
        let mut e_field = Vec2::new(0., 0.);

        for (j, charge) in model.bodies.iter().enumerate() {
            if i != j { e_field += charge.e_field(pos); }
        }
        
        model.bodies[i].update(e_field, dt);
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);

    let dt = update.since_last.as_secs_f32();
    let mut next_state = None;

    match model.state {
        State::Simulating => simulate(model, dt),
        State::ShowGui => make_ui(model),
        State::AddBody(ref name) => {
            let b = model.constructors.get_mut(name).unwrap();
            let ctx = model.egui.begin_frame();
            
            egui::Window::new("Add").show(&ctx, |ui| {
                if b.make_ui(ui) {
                    next_state = Some(State::ShowGui);
                    model.bodies.push(b.get_value());
                }
            });
        }
        _ => {}
    }

    if let Some(s) = next_state { model.state = s; }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let screen = app.window_rect();
    let draw = app.draw();
    
    draw.background().color(BLACK);
    
    for points in &model.isopotentials {
        draw.polyline()
            .points(points.iter().copied())
            .color(WHITE);
    }

    for points in &model.field_lines {
        draw.polyline()
            .points(points.iter().copied())
            .color(YELLOW);
    }

    for c in &model.bodies { c.draw(&draw); }
    
    draw.text(match model.state {
            State::Simulating => "Running",
            State::ShowGui => "Paused",
            State::AddIsopotential => "Adding isopotential",
            State::AddBody(_) => "Adding body"
        })
        .x_y((screen.left()+screen.right())/2., screen.top()-10.);
    
    if let State::AddBody(ref b) = model.state {
        model.constructors[b].get_value().draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
    
    if matches!(model.state, State::ShowGui | State::AddBody(_)) {
        model.egui.draw_to_frame(&frame).unwrap();
    }
}

