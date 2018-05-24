use shape::{Shape, Point};
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
pub struct Triangle {
    points: Vec<Point>
}

impl Shape for Triangle {
    fn new(range_x: &u32, range_y: &u32) -> Vec<Point> {
        let mut rng = thread_rng();
        let n_tiles = 4;
        let x_index_generator = Range::new(0, n_tiles);
        let y_index_generator = Range::new(0, n_tiles);
        let (tile_x, tile_y) = (
            x_index_generator.ind_sample(&mut rng),
            y_index_generator.ind_sample(&mut rng)
        );
        let tile_size_x = range_x / n_tiles;
        let tile_size_y = range_y / n_tiles;
        let (x_coord_generator, y_coord_generator) = (
            Range::new(tile_size_x * tile_x, tile_size_x * (tile_x + 1)),
            Range::new(tile_size_y * tile_y, tile_size_y * (tile_y + 1))
        );
        // let x_generator = Range::new(img_tile * tile_size_x, (img_tile + 1) * tile_size_x);
        // let y_generator = Range::new(img_tile * tile_size_y, (img_tile + 1) * tile_size_y);
        let p1 = Point {
            x: x_coord_generator.ind_sample(&mut rng),
            y: y_coord_generator.ind_sample(&mut rng),
        };
        let p2 = Point {
            x: x_coord_generator.ind_sample(&mut rng),
            y: y_coord_generator.ind_sample(&mut rng),
        };
        let p3 = Point {
            x: x_coord_generator.ind_sample(&mut rng),
            y: y_coord_generator.ind_sample(&mut rng),
        };
        // let p3 = Point {
        //     x: (p1.x + p2.x) / 2;
        //     y: (p1.y + p2.y) / 2;
        // };
        // println!("range: {}, {}", range_x, range_y);
        // println!("points: {:?}, {:?}, {:?}", p1, p2, p3);
        let points = vec![p1, p2, p3];
        points
    }

    fn center(points: &Vec<Point>) -> Point {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_within_range() {
        let points = Triangle::new(&512, &512);
        for point in &points {
            assert!(point.x < 512);
            assert!(point.y < 512);
        }
    }
}
