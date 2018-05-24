use image_utils::{rgba_to_str};
use image::{self, Rgba};
use triangle::Triangle;
use rectangle::Rectangle;
use std::fmt;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum Shapes {
    Rectangle,
    Triangle,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone)]
pub struct Polygon {
    pub points: Vec<Point>,
    range_x: u32,
    range_y: u32,
    shape: Shapes,
}

fn deg2rad(deg: &f32) -> f32 {
    deg * PI / 180.0
}

impl Polygon {
    pub fn new(shape: Shapes, range_x: u32, range_y: u32) -> Polygon {
        match shape {
            Shapes::Rectangle => Polygon {
                points: Rectangle::new(&range_x, &range_y), range_x: range_x, range_y: range_y, shape: shape
            },
            Shapes::Triangle => Polygon {
                points: Triangle::new(&range_x, &range_y), range_x: range_x, range_y: range_y, shape: shape
            },
            _ => Polygon {
                points: Vec::new(), range_x: range_x, range_y: range_y, shape: Shapes::Rectangle
            },
        }
    }

    fn clamp_values(&mut self) {
        for point in &mut self.points {
            if point.x >= self.range_x {
                point.x = self.range_x - 1;
            }
            if point.y >= self.range_y {
                point.y = self.range_y - 1;
            }
        }
    }

    pub fn svg<T>(&self, color: &Rgba<T>) -> String
        where T: fmt::Display + image::Primitive {
        let mut points_str = String::new();
        for point in &self.points {
            // println!("{},{}", point.x, point.y);
            points_str.push_str(&format!("{},{} ", point.x, point.y));
        }
        format!(
            "<polygon points=\"{}\" fill=\"rgb({})\" />",
            points_str, rgba_to_str(&color)
        )
    }

    pub fn get_bounds(&self) -> [Point; 2] {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;
        for point in &self.points {
            if point.x > max_x {
                max_x = point.x;
            } else if point.x < min_x {
                min_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            } else if point.y < min_y {
                min_y = point.y;
            }
        }
        let min = Point { x: min_x, y: min_y };
        let max = Point { x: max_x, y: max_y };
        [min, max]
    }

    fn center(&self) -> Point {
        match self.shape {
            Shapes::Rectangle => Rectangle::center(&self.points),
            _ => self.points[0].clone(),
        }
    }

    pub fn rotate(&mut self, deg: &f32) {
        let radians = deg2rad(deg);
        let sin = radians.sin();
        let cos = radians.cos();
        let rotation_point = self.points[0].clone();
        let r_x = rotation_point.x as f32;
        let r_y = rotation_point.y as f32;
        for point in &mut self.points {
            let x = (point.x as f32 - r_x) as f32 * cos - (point.y as f32 - r_y) as f32 * sin;
            let y = (point.x as f32 - r_x) as f32 * sin + (point.y as f32 - r_y) as f32 * cos;
            point.x = (x.floor() + r_x) as u32;
            point.y = (y.floor() + r_y) as u32;
        };
        self.clamp_values();
    }

    pub fn scale(&mut self, scale_x: &u32, scale_y: &u32) {
        for point in &mut self.points {
            point.x *= scale_x;
            point.y *= scale_y;
        }
        self.clamp_values();
    }

    // fn translate_to_origin(arg: Type) -> RetType {
    //     unimplemented!();
    // }
}

pub trait Shape {
    fn new(range_x: &u32, range_y: &u32) -> Vec<Point>;
    fn center(points: &Vec<Point>) -> Point;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_rotate_triangle() {
        // let polygon = Polygon::new("triangle", 512, 512);
        // polygon.points.push();

        // for point in &polygon.points {
        //     assert!(point.x < 512);
        //     assert!(point.y < 512);
        // }
    }
}
