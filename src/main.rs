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
    trace_origin: Option<Vec2>,
    field_lines: bool
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
        trace_origin: None,
        field_lines: false
    };

    for _ in 0..3 {
        model.bodies.push(Box::new(PointCharge {
            charge: rng.gen_range(-100.0..100.0),
            mass: 1.,
            pos: Vec2::new(
                rng.gen_range(screen.left()..screen.right()),
                rng.gen_range(screen.bottom()..screen.top())
            ),
            vel: Vec2::ZERO
        }));

        model.bodies.push(Box::new(Dipole::new(
            10.,
            1.,
            Vec2::new(
                rng.gen_range(screen.left()..screen.right()),
                rng.gen_range(screen.bottom()..screen.top())
            ),
            Vec2::ZERO
        )));
    }

    model
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.running = !model.running,
        Key::F => model.field_lines = !model.field_lines,
        _ => {}
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    let pos = Vec2::new(app.mouse.x, app.mouse.y);

    match button {
        MouseButton::Left => model.trace_origin = Some(pos),
        MouseButton::Right => model.trace_origin = None,
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
    
    if let Some(trace_origin) = model.trace_origin {
        let points = util::isopotential_points(
            &model.bodies, trace_origin,
            5e-3, 5., 1e-3, 1000
        );
        
        draw.polyline()
            .points(points.iter().copied())
            .color(WHITE);

        if model.field_lines {
            let fields: Vec<f32> = points.iter()
                                         .map(|r| model.bodies.e_field(*r).length())
                                         .collect();
            let flux_step = 10.;
            let mut flux = 0.;
            let mut field_line_origins = Vec::new();

            for i in 0..points.len() {
                let i_next = (i+1)%points.len();
                let dist = (points[i]-points[i_next]).length();
                let avg_field_strength = (fields[i]+fields[i_next])/2.;
                let flux_to_next = dist*avg_field_strength;
                
                flux += flux_to_next;

                if flux > flux_step {
                    flux -= flux_step;
                    let frac = 1.-flux/flux_to_next;

                    field_line_origins.push(points[i].lerp(points[i_next], frac));
                }
            }
            
            for origin in field_line_origins {
                let points = util::field_line_points(
                    &model.bodies, origin,
                    5e-3, 5., 1e-3, 1000
                );

                draw.polyline()
                    .points(points.into_iter())
                    .color(YELLOW);
            }
        }
    }

    for c in &model.bodies { c.draw(&draw); }
    
    draw.text(if model.running { "Running" } else { "Paused" })
        .x_y(screen.left()+30., screen.top()-10.);

    draw.to_frame(app, &frame).unwrap();
}

