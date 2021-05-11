pub mod rk4th;
use rk4th::rk4th3d::RungeKutta4thOrder2nd;

pub mod vector;
use crate::vector::{Point3, Vector3};

fn gravity() -> Vector3 {
    [0., 0., -9.8].into()
}

#[derive(Debug)]
pub struct Particle {
    pub mass: f64,
    pub position: Point3,
    pub velocity: Vector3,
    delta_time: f64,
}

impl Particle {
    pub fn new(mass: f64, position: Point3) -> Self {
        Self {
            mass,
            position,
            velocity: Vector3::zeros(),
            delta_time: 0.01,
        }
    }

    pub fn set_delta_time(&mut self, delta_time: f64) {
        self.delta_time = delta_time;
    }
}

impl RungeKutta4thOrder2nd for Particle {
    fn delta_time(&self) -> f64 {
        self.delta_time
    }

    fn cal_ddy(_t: f64, _v: &Vector3, _x: &Vector3) -> Vector3 {
        gravity()
    }

    fn cal_dy(_t: f64, v: &Vector3, _x: &Vector3) -> Vector3 {
        v.clone()
    }

    fn current_value(&self) -> (&Vector3, &Vector3) {
        (&self.velocity, &self.position.coords)
    }

    fn update(&mut self, vs: &[Vector3]) {
        self.velocity += vs[0];
        self.position += vs[1];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
