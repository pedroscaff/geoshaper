extern crate geoshaper;

use std::fs::remove_file;
use std::path::Path;

#[test]
fn smoke() {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let img_path = root_dir.join(Path::new("lena_std.tif"));
    let result_path = root_dir.join(Path::new("result.png"));

    let mut o = geoshaper::simulation::Options::default();
    o.max_iter = 10;
    o.render_debug_rasters = false;

    assert_eq!(geoshaper::run(&img_path, Some(o)).is_ok(), true);
    assert_eq!(result_path.exists(), true);
    assert_eq!(remove_file(result_path).is_ok(), true);
}
