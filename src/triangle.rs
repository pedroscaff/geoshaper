use shape::{Shape, Point, Polygon};
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
pub struct Triangle {
    points: Vec<Point>
}

impl Shape for Triangle {
    fn new(range_x: &u32, range_y: &u32) -> Polygon {
        let x_generator = Range::new(0, range_x + 1);
        let y_generator = Range::new(0, range_y + 1);
        let mut rng = thread_rng();
        let p1 = Point {
            x: x_generator.ind_sample(&mut rng),
            y: y_generator.ind_sample(&mut rng),
        };
        let p2 = Point {
            x: x_generator.ind_sample(&mut rng),
            y: y_generator.ind_sample(&mut rng),
        };
        let p3 = Point {
            x: x_generator.ind_sample(&mut rng),
            y: y_generator.ind_sample(&mut rng),
        };
        let points = vec![p1, p2, p3];
        Polygon { points: points }
    }
}