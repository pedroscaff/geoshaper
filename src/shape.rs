use image_utils::{rgba_to_str};
use image::{self, Rgba};
use triangle::Triangle;
use rectangle::Rectangle;
use std::fmt;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum Shapes {
    Rectangle,
    Triangle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Point>,
    range_x: f32,
    range_y: f32,
    shape: Shapes,
    fill_color: Rgba<u8>
}

fn deg2rad(deg: &f32) -> f32 {
    deg * PI / 180.0
}

impl Polygon {
    pub fn new(shape: Shapes, range_x: f32, range_y: f32) -> Polygon {
        let default_white_color = Rgba {
            data: [255 as u8, 255 as u8, 255 as u8, 255]
        };

        match shape {
            Shapes::Rectangle => Polygon {
                points: Rectangle::new(&range_x, &range_y), range_x: range_x, range_y: range_y, shape: shape, fill_color: default_white_color
            },
            Shapes::Triangle => Polygon {
                points: Triangle::new(&range_x, &range_y), range_x: range_x, range_y: range_y, shape: shape, fill_color: default_white_color
            }
        }
    }

    pub fn set_fill_color(&mut self, color: Rgba<u8>) {
        self.fill_color = color;
    }

    pub fn svg(&self) -> String {
        let mut points_str = String::new();
        for point in &self.points {
            // println!("{},{}", point.x, point.y);
            points_str.push_str(&format!("{},{} ", point.x, point.y));
        }
        format!(
            "<polygon points=\"{}\" fill=\"rgb({})\" fill-opacity=\"0.7\"/>",
            points_str, rgba_to_str(&self.fill_color)
        )
    }

    pub fn get_bounds(&self) -> [Point; 2] {
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        let mut min_x = 0.0;
        let mut min_y = 0.0;
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

    pub fn rotate(&mut self, deg: &f32) {
        let radians = deg2rad(deg);
        let sin = radians.sin();
        let cos = radians.cos();
        let rotation_point = self.center();
        let r_x = rotation_point.x;
        let r_y = rotation_point.y;
        debug!("rotation degree: {}", deg);
        debug!("rotation center: {:?}", rotation_point);
        debug!("values before rotation: {:?}", self.points);
        for point in &mut self.points {
            let x = (point.x - r_x) * cos - (point.y - r_y) * sin;
            let y = (point.x - r_x) * sin + (point.y - r_y) * cos;
            point.x = x + r_x;
            point.y = y + r_y;
        };
        debug!("values after rotation: {:?}", self.points);
        self.clamp_values();
    }

    pub fn scale(&mut self, scale_x: &f32, scale_y: &f32) {
        // for point in &mut self.points {
        //     point.x *= scale_x;
        //     point.y *= scale_y;
        // }
        // self.clamp_values();
        match self.shape {
            Shapes::Rectangle => Rectangle::scale(&mut self.points, scale_x, scale_y),
            _ => (),
        };
        self.clamp_values();
    }

    fn center(&self) -> Point {
        match self.shape {
            Shapes::Rectangle => Rectangle::center(&self.points),
            _ => self.points[0].clone(),
        }
    }

    fn clamp_values(&mut self) {
        for point in &mut self.points {
            if point.x >= self.range_x {
                debug!("clamping value for shape: from: {}, to: {}", point.x, self.range_x);
                point.x = self.range_x - 1.0;
            } else if point.x < 0.0 {
                debug!("clamping value for shape: from: {}, to: {}", point.x, 0.0);
                point.x = 0.0;
            }
            if point.y >= self.range_y {
                debug!("clamping value for shape: from: {}, to: {}", point.y, self.range_y);
                point.y = self.range_y - 1.0;
            } else if point.y < 0.0 {
                debug!("clamping value for shape: from: {}, to: {}", point.y, 0.0);
                point.y = 0.0;
            }
        }
    }
}

pub trait Shape {
    fn new(range_x: &f32, range_y: &f32) -> Vec<Point>;
    fn center(points: &Vec<Point>) -> Point;
    fn scale(points: &mut Vec<Point>, scale_x: &f32, scale_y: &f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_rotate_rectangle() {
        // let polygon = Polygon::new("triangle", 512, 512);
        // polygon.points.push();

        // for point in &polygon.points {
        //     assert!(point.x < 512);
        //     assert!(point.y < 512);
        // }
    }
}
