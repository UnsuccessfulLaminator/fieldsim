mod bodies;
mod util;

use nannou::prelude::*;
use rand::Rng;
use bodies::*;



fn main() {
    nannou::app(model)
           .loop_mode(LoopMode::RefreshSync)
           .update(update)
           .run();
}

struct Model {
    running: bool,
    bodies: Vec<Box<dyn Body>>,
    isopotentials: Vec<Vec<Vec2>>,
    field_lines: Vec<Vec<Vec2>>
}

fn model(app: &App) -> Model {
    app.new_window()
       .key_pressed(key_pressed)
       .mouse_pressed(mouse_pressed)
       .view(view)
       .build()
       .unwrap();

    let screen = app.window_rect();
    let mut rng = rand::thread_rng();
    let mut model = Model {
        running: false,
        bodies: Vec::new(),
        isopotentials: Vec::new(),
        field_lines: Vec::new()
    };

    for _ in 0..10 {
        model.bodies.push(Box::new(PointCharge {
            charge: rng.gen_range(-100.0..100.0),
            mass: 1.,
            pos: Vec2::new(
                rng.gen_range(screen.left()..screen.right()),
                rng.gen_range(screen.bottom()..screen.top())
            ),
            vel: Vec2::ZERO
        }));

        /*model.bodies.push(Box::new(Dipole::new(
            10.,
            1.,
            Vec2::new(
                rng.gen_range(screen.left()..screen.right()),
                rng.gen_range(screen.bottom()..screen.top())
            ),
            Vec2::ZERO
        )));*/
    }

    model
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.running = !model.running,
        Key::F => {
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
        _ => {}
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    let pos = Vec2::new(app.mouse.x, app.mouse.y);

    match button {
        MouseButton::Left => {
            let (mut points, is_loop) = util::isopotential_points(
                &model.bodies, pos,
                5e-3, 5., 1e-3, 1000
            );
            
            if is_loop { points.push(points[0]); }

            model.isopotentials.push(points);
        }
        MouseButton::Right => {
            model.isopotentials.clear();
            model.field_lines.clear();
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if !model.running { return; }

    let dt = update.since_last.as_secs_f32();
    
    for i in 0..model.bodies.len() {
        let pos = model.bodies[i].pos();
        let mut e_field = Vec2::new(0., 0.);

        for (j, charge) in model.bodies.iter().enumerate() {
            if i != j { e_field += charge.e_field(pos); }
        }
        
        model.bodies[i].update(e_field, dt);
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
    
    draw.text(if model.running { "Running" } else { "Paused" })
        .x_y(screen.left()+30., screen.top()-10.);

    draw.to_frame(app, &frame).unwrap();
}

