use std::rc::Rc;
use std::cell::RefCell;
use std::num::One;
use kiss3d::window;
use kiss3d::object::Object;
use nalgebra::na::{Vec3, Iso3, Transformation, Rotation};
use nalgebra::na;
use nphysics::object::RigidBody;

pub struct Cone {
    priv color:      Vec3<f32>,
    priv base_color: Vec3<f32>,
    priv delta:      Iso3<f32>,
    priv gfx:        Object,
    priv body:       Rc<RefCell<RigidBody>>,
}

impl Cone {
    pub fn new(body:   Rc<RefCell<RigidBody>>,
               delta:  Iso3<f32>,
               r:      f32,
               h:      f32,
               color:  Vec3<f32>,
               window: &mut window::Window) -> Cone {
        let mut realign: Iso3<f32> = One::one();
        let _frac_pi_2: f32 = Float::frac_pi_2();
        realign.append_rotation(&Vec3::new(0.0, 0.0, -_frac_pi_2));

        let mut res = Cone {
            color:      color,
            base_color: color,
            delta:      delta * realign,
            gfx:        window.add_cone(h as f32, r as f32),
            body:       body
        };

        res.gfx.set_color(color.x, color.y, color.z);
        res.update();

        res
    }

    pub fn select(&mut self) {
        self.color = Vec3::x();
    }

    pub fn unselect(&mut self) {
        self.color = self.base_color;
    }

    pub fn update(&mut self) {
        let rb = self.body.borrow();

        if rb.is_active() {
            self.gfx.set_transformation(na::transformation(rb.deref()) * self.delta);
            self.gfx.set_color(self.color.x, self.color.y, self.color.z);
        }
        else {
            self.gfx.set_color(self.color.x * 0.25, self.color.y * 0.25, self.color.z * 0.25);
        }
    }

    pub fn object<'r>(&'r self) -> &'r Object {
        &self.gfx
    }
}
