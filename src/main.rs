mod shapes;
use shapes::{Area, Circle, React};
use std::fmt::Debug;

struct ReactIterator {
    points: Vec<(f64, f64)>,
    index: usize
}

impl Iterator for ReactIterator {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.index;
        self.index += 1;
        return self.points.get(idx).map(|x| *x)
    }
}

fn main() {
    let react = React {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 20.0,
    };
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    println!("default circle {:?}", Circle::default());
    println!("Default React {:?}", React::default());
    println!("React area: {}", react.area());
    println!("Circle area: {}", circle.area());
}
