use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;
use shape::{Point, Shape};

/**
 * p0--p1
 * |   |
 * |   |
 * |   |
 * p3--p2
 */
#[derive(Debug)]
pub struct Rectangle {
    points: Vec<Point>,
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
            y: (points[3].y + points[0].y) / 2.0,
        }
    }

    fn scale(points: &mut Vec<Point>, scale_x: &f32, scale_y: &f32) {
        let current_width = points[1].x - points[0].x;
        let current_height = points[3].y - points[0].y;
        let scaled_width = points[0].x + current_width * scale_x;
        let scaled_height = points[0].y + current_height * scale_y;
        points[1].x = scaled_width;
        points[2].x = scaled_width;
        points[2].y = scaled_height;
        points[3].y = scaled_height;
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

    #[test]
    fn should_scale() {
        let p0 = Point { x: 0.0, y: 0.0 };
        let p1 = Point { x: 5.0, y: 0.0 };
        let p2 = Point { x: 5.0, y: 5.0 };
        let p3 = Point { x: 0.0, y: 5.0 };
        let mut points = vec![p0, p1, p2, p3];
        Rectangle::scale(&mut points, &2.0, &2.0);
        // should remain the same
        assert!(points[0].x == 0.0);
        assert!(points[0].y == 0.0);
        // x should change
        assert!(points[1].x == 10.0);
        assert!(points[1].y == 0.0);
        // both should change
        assert!(points[2].x == 10.0);
        assert!(points[2].y == 10.0);
        // y should change
        assert!(points[3].x == 0.0);
        assert!(points[3].y == 10.0);
    }
}
