use shape::{Shape, Point};
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
pub struct Rectangle {
    points: Vec<Point>
}

impl Shape for Rectangle {
    fn new(range_x: &u32, range_y: &u32) -> Vec<Point> {
        let mut rng = thread_rng();
        let base_width = range_x / 16;
        let base_height = range_y / 16;
        let x_coord_generator = Range::new(0, *range_x);
        let y_coord_generator = Range::new(0, *range_y);
        let p1 = Point {
            x: x_coord_generator.ind_sample(&mut rng),
            y: y_coord_generator.ind_sample(&mut rng),
        };
        let p2 = Point {
            x: p1.x + base_width,
            y: p1.y,
        };
        let p3 = Point {
            x: p1.x + base_width,
            y: p1.y + base_height,
        };
        let p4 = Point {
            x: p1.x,
            y: p1.y + base_height,
        };
        // println!("points: {:?}, {:?}, {:?}, {:?}", p1, p2, p3, p4);
        let mut points = vec![p1, p2, p3, p4];
        for point in &mut points {
            if point.x >= *range_x {
                point.x = range_x - 1;
            }
            if point.y >= *range_y {
                point.y = range_y - 1;
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_within_range() {
        let points = Rectangle::new(&512, &512);
        for point in &points {
            assert!(point.x < 512);
            assert!(point.y < 512);
        }
    }
}
