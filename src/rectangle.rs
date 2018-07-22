use shape::{Shape, Point};
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};


/**
 * p0--p1
 * |   |
 * |   |
 * |   |
 * p3--p2
 */
#[derive(Debug)]
pub struct Rectangle {
    points: Vec<Point>
}

impl Shape for Rectangle {
    fn new(range_x: &f32, range_y: &f32) -> Vec<Point> {
        let mut rng = thread_rng();
        let base_width = range_x / 8.0;
        let base_height = range_y / 8.0;
        let x_coord_generator = Range::new(0.0, *range_x - base_width);
        let y_coord_generator = Range::new(0.0, *range_y - base_height);
        let p0 = Point {
            x: x_coord_generator.ind_sample(&mut rng),
            y: y_coord_generator.ind_sample(&mut rng),
        };
        let p1 = Point {
            x: p0.x + base_width,
            y: p0.y,
        };
        let p2 = Point {
            x: p0.x + base_width,
            y: p0.y + base_height,
        };
        let p3 = Point {
            x: p0.x,
            y: p0.y + base_height,
        };
        // println!("points: {:?}, {:?}, {:?}, {:?}", p1, p2, p3, p4);
        let points = vec![p0, p1, p2, p3];
        points
    }

    fn center(points: &Vec<Point>) -> Point {
        Point {
            x: (points[1].x + points[0].x) / 2.0,
            y: (points[3].y + points[0].y) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_within_range() {
        let points = Rectangle::new(&512.0, &512.0);
        for point in &points {
            assert!(point.x < 512.0);
            assert!(point.y < 512.0);
        }
    }
}
