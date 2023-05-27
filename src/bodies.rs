use nannou::draw::Draw;
use nannou::prelude::*;
use std::ops::DerefMut;



pub trait Body {
    fn pos(&self) -> Vec2;
    fn e_field(&self, pos: Vec2) -> Vec2;
    fn potential(&self, pos: Vec2) -> f32;
    fn update(&mut self, e_field: Vec2, dt: f32);
    fn draw(&self, draw: &Draw);
}



pub struct PointCharge {
    pub charge: f32,
    pub mass: f32,
    pub pos: Vec2,
    pub vel: Vec2
}

impl Body for PointCharge {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn e_field(&self, pos: Vec2) -> Vec2 {
        let r = pos-self.pos;
        
        r*self.charge/r.length_squared()
    }

    fn potential(&self, pos: Vec2) -> f32 {
        -self.charge*(pos-self.pos).length().ln()
    }

    fn update(&mut self, e_field: Vec2, dt: f32) {
        let dv = dt*e_field*self.charge/self.mass;
        
        self.pos += (self.vel+0.5*dv)*dt;
        self.vel += dv;
    }

    fn draw(&self, draw: &Draw) {
        let r = (1.-(-self.charge.abs()).exp())*5.;

        draw.ellipse()
            .color(if self.charge < 0. { BLUE } else { RED})
            .xy(self.pos)
            .radius(r);

        draw.line()
            .start(self.pos-Vec2::new(r, 0.))
            .end(self.pos+Vec2::new(r, 0.))
            .stroke_weight(r/5.)
            .color(WHITE);
        
        if self.charge >= 0. {
            draw.line()
                .start(self.pos-Vec2::new(0., r))
                .end(self.pos+Vec2::new(0., r))
                .stroke_weight(r/5.)
                .color(WHITE);
        }
    }
}



pub struct Dipole {
    q1: PointCharge,
    q2: PointCharge,
    pos: Vec2,
}

impl Dipole {
    pub fn new(dipole: f32, mass: f32, pos: Vec2, vel: Vec2) -> Dipole {
        Dipole {
            q1: PointCharge {
                charge: dipole,
                mass: mass/2.,
                pos: pos+Vec2::new(0.5, 0.),
                vel: vel
            },
            q2: PointCharge {
                charge: -dipole,
                mass: mass/2.,
                pos: pos-Vec2::new(0.5, 0.),
                vel: vel
            },
            pos: pos
        }
    }
}

impl Body for Dipole {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn e_field(&self, pos: Vec2) -> Vec2 {
        self.q1.e_field(pos)+self.q2.e_field(pos)
    }

    fn potential(&self, pos: Vec2) -> f32 {
        self.q1.potential(pos)+self.q2.potential(pos)
    }

    fn update(&mut self, e_field: Vec2, dt: f32) {
        ; // todo
    }

    fn draw(&self, draw: &Draw) {
        let r = (1.-(-self.q1.charge).exp())*5.;
        let forward = r*(self.q1.pos-self.pos)*2.;
        let side = forward.perp()/2.;
        
        draw.tri()
            .color(RED)
            .points(self.pos+forward, self.pos+side, self.pos-side);
        
        draw.tri()
            .color(BLUE)
            .points(self.pos-forward, self.pos-side, self.pos+side);
    }
}



pub struct CircleCharge {
    pub charge: f32,
    pub mass: f32,
    pub radius: f32,
    pub pos: Vec2,
    pub vel: Vec2
}

impl Body for CircleCharge {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn e_field(&self, pos: Vec2) -> Vec2 {
        let r = pos-self.pos;
        let r_sq = r.length_squared();
        let r0_sq = self.radius*self.radius;

        if r_sq >= r0_sq { r*self.charge/r_sq }
        else { r*self.charge/r0_sq }
    }

    fn potential(&self, pos: Vec2) -> f32 {
        let r = pos-self.pos;
        let r_sq = r.length_squared();
        let r0_sq = self.radius*self.radius;

        if r_sq >= r0_sq { -self.charge*r_sq/(2.*r0_sq) }
        else { ((r0_sq/r_sq).ln()-1.)/2. }
    }

    fn update(&mut self, e_field: Vec2, dt: f32) {
        let dv = dt*e_field*self.charge/self.mass;
        
        self.pos += (self.vel+0.5*dv)*dt;
        self.vel += dv;
    }

    fn draw(&self, draw: &Draw) {
        let d = draw.ellipse().radius(self.radius).xy(self.pos);

        if self.charge < 0. { d.rgba(0., 0., 1., 0.5); }
        else { d.rgba(1., 0., 0., 0.5); }
    }
}



pub struct GlobalField {
    pub field: Vec2
}

impl Body for GlobalField {
    fn pos(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn e_field(&self, _pos: Vec2) -> Vec2 {
        self.field
    }

    fn potential(&self, pos: Vec2) -> f32 {
        -pos.dot(self.field)
    }

    fn update(&mut self, _e_field: Vec2, _dt: f32) {}
    fn draw(&self, _draw: &Draw) {}
}



impl<C: DerefMut<Target=[Box<dyn Body>]>> Body for C {
    fn pos(&self) -> Vec2 {
        self.iter().fold(Vec2::ZERO, |acc, b| acc+b.pos())/self.len() as f32
    }

    fn e_field(&self, pos: Vec2) -> Vec2 {
        self.iter().fold(Vec2::ZERO, |acc, b| acc+b.e_field(pos))
    }

    fn potential(&self, pos: Vec2) -> f32 {
        self.iter().map(|b| b.potential(pos)).sum()
    }

    fn update(&mut self, e_field: Vec2, dt: f32) {
        for b in self.iter_mut() { b.update(e_field, dt); }
    }

    fn draw(&self, draw: &Draw) {
        for b in self.iter() { b.draw(draw); }
    }
}
