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
        
        r*self.charge/r.length().powi(3)
    }

    fn potential(&self, pos: Vec2) -> f32 {
        self.charge/(pos-self.pos).length()
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
    pub dipole: f32,
    pub inertia: f32,
    pub pos: Vec2,
    pub angle: f32,
    pub ang_vel: f32,
}

impl Body for Dipole {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn e_field(&self, pos: Vec2) -> Vec2 {
        let r = pos-self.pos;
        let r_mag = r.length();
        let r_unit = r/r_mag;
        let p = self.dipole*Vec2::new(self.angle.cos(), self.angle.sin());

        (3.*p.dot(r_unit)*r_unit-self.dipole)/r_mag.powi(3)
    }

    fn potential(&self, pos: Vec2) -> f32 {
        let r = pos-self.pos;
        let p = self.dipole*Vec2::new(self.angle.cos(), self.angle.sin());

        p.dot(r)/r.length().powi(3)
    }

    fn update(&mut self, e_field: Vec2, dt: f32) {
        let p = self.dipole*Vec2::new(self.angle.cos(), self.angle.sin());
        let torque = p.perp_dot(e_field);
        let dw = torque*dt;

        self.angle += (self.ang_vel+0.5*dw)*dt;
        self.ang_vel += dw;
    }

    fn draw(&self, draw: &Draw) {
        let r = (1.-(-self.dipole).exp())*5.;
        let forward = r*Vec2::new(self.angle.cos(), self.angle.sin());
        let side = forward.perp()/2.;
        
        draw.tri()
            .color(RED)
            .points(self.pos+forward, self.pos+side, self.pos-side);
        
        draw.tri()
            .color(BLUE)
            .points(self.pos-forward, self.pos-side, self.pos+side);
    }
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
