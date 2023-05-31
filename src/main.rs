mod bodies;
mod util;

use nannou::prelude::*;
use nannou::winit;
use nannou_egui::{egui, Egui};
use rand::Rng;
use std::time::Duration;
use bodies::*;



fn main() {
    nannou::app(model)
           .loop_mode(LoopMode::RefreshSync)
           .update(update)
           .run();
}

#[derive(PartialEq)]
enum State {
    Simulating,
    ShowGui,
    AddIsopotential
}

struct Model {
    state: State,
    bodies: Vec<Box<dyn Body>>,
    isopotentials: Vec<Vec<Vec2>>,
    field_lines: Vec<Vec<Vec2>>,
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
    let mut rng = rand::thread_rng();
    let mut model = Model {
        state: State::ShowGui,
        bodies: Vec::new(),
        isopotentials: Vec::new(),
        field_lines: Vec::new(),
        egui: Egui::from_window(&window)
    };

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
    if model.state == State::ShowGui {
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
            if model.state == State::AddIsopotential {
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

fn make_ui(model: &mut Model, elapsed: Duration) {
    model.egui.set_elapsed_time(elapsed);

    let ctx = model.egui.begin_frame();

    egui::Window::new("Bodies").show(&ctx, |ui| {
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
    make_ui(model, update.since_start);

    if model.state == State::Simulating {
        simulate(model, update.since_last.as_secs_f32());
    }
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
            State::AddIsopotential => "Adding isopotential"
        })
        .x_y((screen.left()+screen.right())/2., screen.top()-10.);

    draw.to_frame(app, &frame).unwrap();
    
    if model.state == State::ShowGui {
        model.egui.draw_to_frame(&frame).unwrap();
    }
}

