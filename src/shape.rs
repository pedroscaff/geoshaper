use triangle::Triangle;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Point>
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.svg())
    }
}

impl Polygon {
    pub fn new(shape: &str, range_x: &u32, range_y: &u32) -> Polygon {
        match shape {
            "triangle" => Triangle::new(range_x, range_y),
            _ => Triangle::new(range_x, range_y),
        }
    }

    pub fn svg(&self) -> String {
        let mut points_str = String::new();
        for point in &self.points {
            points_str.push_str(&format!("{},{} ", point.x, point.y));
        }
        format!(
            "<polygon points=\"{}\" fill=\"{}\" />",
            points_str, "blue"
        )
    }
}

pub trait Shape {
    fn new(range_x: &u32, range_y: &u32) -> Polygon;
}
