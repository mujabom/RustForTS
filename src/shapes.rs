use std::f64::consts::PI;
use std::fmt::{Debug, Formatter};

pub struct React {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for React {
    fn area(&self) -> f64 {
        return self.height * self.width;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            x: 0.0,
            y: 0.0,
            radius: 20.0,
        }
    }
}
impl Default for React {
    fn default() -> Self {
        React {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        }
    }
}
impl Debug for React {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "React {{ x: {}, y: {}, width: {}, height: {} }}",
            self.x, self.y, self.width, self.height
        )
    }
}
impl Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle {{ x: {}, y: {}, radius is no so cool: {} }}",
            self.x, self.y, self.radius
        )
    }
}
