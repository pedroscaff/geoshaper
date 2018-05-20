use image_utils::{get_average_color_from_area, rgba_to_str};
use image::{DynamicImage, GenericImage, Rgba};
use std::sync::Arc;
use triangle::Triangle;
use std::fmt;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone)]
pub struct Polygon {
    pub points: Vec<Point>,
    img: Arc<DynamicImage>,
    range_x: u32,
    range_y: u32,
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.svg())
    }
}

fn deg2rad(deg: &f32) -> f32 {
    deg * PI / 180.0
}

impl Polygon {
    pub fn new(img: Arc<DynamicImage>, shape: &str, range_x: u32, range_y: u32) -> Polygon {
        match shape {
            "triangle" => Polygon {
                img: img, points: Triangle::new(&range_x, &range_y), range_x: range_x, range_y: range_y
            },
            _ => Polygon {
                img: img, points: Triangle::new(&range_x, &range_y), range_x: range_x, range_y: range_y
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

    pub fn svg(&self) -> String {
        let mut points_str = String::new();
        for point in &self.points {
            // println!("{},{}", point.x, point.y);
            points_str.push_str(&format!("{},{} ", point.x, point.y));
        }
        let color = get_average_color_from_area(self.img.clone(), self.get_bounds());
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

    pub fn rotate(&mut self, deg: &f32) {
        let radians = deg2rad(deg);
        let sin = radians.sin();
        let cos = radians.cos();
        for point in &mut self.points {
            let x = point.x as f32 * cos - point.y as f32 * sin;
            let y = point.x as f32 * sin + point.y as f32 * cos;
            point.x = x.floor() as u32;
            point.y = y.floor() as u32;
        };
        self.clamp_values();
    }
}

pub trait Shape {
    fn new(range_x: &u32, range_y: &u32) -> Vec<Point>;
}
