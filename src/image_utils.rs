use image::{self, DynamicImage, GenericImage, ImageResult, Rgba, RgbaImage};
use shape::Point;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

pub fn rgba_to_str<T>(color: &Rgba<T>) -> String
where
    T: fmt::Display + image::Primitive,
{
    format!("{},{},{}", color.data[0], color.data[1], color.data[2])
    // 153)
    // color.data[3])
}

pub fn load_image(p: &Path) -> ImageResult<DynamicImage> {
    image::open(p)
}

pub fn get_average_color_from_area(img: Arc<DynamicImage>, bounds: [Point; 2]) -> Rgba<u8> {
    check_bounds(&bounds);
    let mut r_sum: u32 = 0;
    let mut g_sum: u32 = 0;
    let mut b_sum: u32 = 0;
    let mut count: u32 = 0;
    for x in bounds[0].x as u32..bounds[1].x as u32 {
        for y in bounds[0].y as u32..bounds[1].y as u32 {
            let pixel = img.get_pixel(x, y);
            r_sum += pixel.data[0] as u32;
            g_sum += pixel.data[1] as u32;
            b_sum += pixel.data[2] as u32;
            count += 1;
        }
    }
    let r_avg = r_sum / count;
    let g_avg = g_sum / count;
    let b_avg = b_sum / count;

    Rgba {
        data: [r_avg as u8, g_avg as u8, b_avg as u8, 255],
    }
}

pub fn get_average_color(img: Arc<DynamicImage>) -> Rgba<u8> {
    let mut r_sum: u32 = 0;
    let mut g_sum: u32 = 0;
    let mut b_sum: u32 = 0;
    let mut count: u32 = 0;
    for (_x, _y, pixel) in img.pixels() {
        r_sum += pixel.data[0] as u32;
        g_sum += pixel.data[1] as u32;
        b_sum += pixel.data[2] as u32;
        count += 1;
    }

    let r_avg = r_sum / count;
    let g_avg = g_sum / count;
    let b_avg = b_sum / count;

    Rgba {
        data: [r_avg as u8, g_avg as u8, b_avg as u8, 255],
    }
}

pub fn image_area_diff(img1: Arc<DynamicImage>, img2: &RgbaImage, bounds: &[Point; 2]) -> f32 {
    check_bounds(&bounds);
    let mut total: u64 = 0;
    let mut count: u64 = 0;
    for x in bounds[0].x as u32..bounds[1].x as u32 {
        for y in bounds[0].y as u32..bounds[1].y as u32 {
            let p1 = img1.get_pixel(x, y);
            let p2 = img2.get_pixel(x, y);
            let r1 = p1.data[0] as i32;
            let g1 = p1.data[1] as i32;
            let b1 = p1.data[2] as i32;
            let r2 = p2.data[0] as i32;
            let g2 = p2.data[1] as i32;
            let b2 = p2.data[2] as i32;
            let r_diff: i32 = r1 - r2;
            let g_diff: i32 = g1 - g2;
            let b_diff: i32 = b1 - b2;
            total += ((r_diff * r_diff) + (g_diff * g_diff) + (b_diff * b_diff)) as u64;
            count += 1;
        }
    }
    ((total / count) as f32).sqrt()
}

pub fn image_diff(img1: Arc<DynamicImage>, img2: &RgbaImage) -> f32 {
    let mut total: u64 = 0;
    let mut count: u64 = 0;
    for (x, y, pixel) in img1.pixels() {
        let r1 = pixel.data[0] as i32;
        let g1 = pixel.data[1] as i32;
        let b1 = pixel.data[2] as i32;
        let pixel2 = img2.get_pixel(x, y);
        let r2 = pixel2.data[0] as i32;
        let g2 = pixel2.data[1] as i32;
        let b2 = pixel2.data[2] as i32;
        let r_diff: i32 = r1 - r2;
        let g_diff: i32 = g1 - g2;
        let b_diff: i32 = b1 - b2;
        total +=
            ((r_diff * r_diff) as u64 + (g_diff * g_diff) as u64 + (b_diff * b_diff) as u64) as u64;
        count += 1;
    }
    ((total / count) as f32).sqrt()
}

fn check_bounds(bounds: &[Point; 2]) {
    if (bounds[1].x - bounds[0].x) < 1.0 {
        error!("image_utils: boundaries must be at least than one!");
    }
    if (bounds[1].y - bounds[0].y) < 1.0 {
        error!("image_utils: boundaries must be at least than one!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn get_test_image() -> DynamicImage {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let img_path = root_dir.join(Path::new("lena_std.tif"));
        load_image(&img_path).unwrap()
    }

    #[test]
    fn test_image_diff() {
        let i = Arc::new(get_test_image());
        let score = image_diff(i.clone(), &i.to_rgba());
        assert_eq!(0.0, score);
    }

    #[test]
    fn test_image_area_diff() {
        let i = Arc::new(get_test_image());
        let bounds = &[Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 10.0 }];
        let score = image_area_diff(i.clone(), &i.to_rgba(), bounds);
        assert_eq!(0.0, score);
    }
}
