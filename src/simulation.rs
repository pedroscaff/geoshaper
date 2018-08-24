use std::default::Default;
use image::DynamicImage;
use individual::GImage;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use image_utils::{get_average_color, image_area_diff};
use image::GenericImage;
use individual::Individual;
use shape::{Polygon, Shapes};
use scoped_threadpool::Pool;

use error::Result;

#[derive(Debug)]
pub struct Options {
    pub pop_size: u32,
    pub shape: String,
    pub max_iter: u32,
    pub num_genes: u32,
    pub render_debug_rasters: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options{
            pop_size: 100,
            shape: "rectangle".to_owned(),
            max_iter: 200,
            num_genes: 100,
            render_debug_rasters: false,
        }
    }
}

pub fn run(target: Arc<DynamicImage>, options: Options) -> Result<()> {
    // let mut population = make_population(options.pop_size, target);
    let avg_color = get_average_color(target.clone());
    let (width, height) = target.dimensions();
    let mut result_gene = GImage::new(1, target.clone(), avg_color, width, height);
    let width = width as f32;
    let height = height as f32;

    let shape = match options.shape.as_str() {
        "rectangle" => Shapes::Rectangle,
        "triangle" => Shapes::Triangle,
        _ => Shapes::Rectangle
    };

    let mut evolutions = 0;
    for i in 0..options.max_iter {
        // generate candidate
        let new_shape = Polygon::new(shape, width, height);
        let mut mutations : Vec<GImage> = Vec::new();
        for j in 0..options.num_genes {
            let new_gene = result_gene.mutate(new_shape.clone(), j);
            mutations.push(new_gene);
        }

        let mut best_fitness = Arc::new(Mutex::new(10_000_000.0));
        let mut best_id = Arc::new(Mutex::new(0));
        let mut pool = Pool::new(4);
        pool.scoped(|scoped| {
            for gene in &mutations {
                let best_fitness = best_fitness.clone();
                let best_id = best_id.clone();

                scoped.execute(move || {
                    let fitness = gene.fitness_mutation();
                    let mut best_fitness = best_fitness.lock().unwrap();
                    let mut best_id = best_id.lock().unwrap();

                    if fitness < *best_fitness {
                        *best_fitness = fitness;
                        *best_id = gene.id();
                    }
                });
            }
        });

        let winner_gene = mutations.iter().find(|ref mutation| mutation.id() == *best_id.lock().unwrap()).unwrap();
        debug!("we have a winner: {}", winner_gene.get_last_polygon().svg());

        let mutation_area_current_fitness = image_area_diff(
            target.clone(),
            &result_gene.as_rgba_img().unwrap(),
            winner_gene.mutation_area()
        );

        // no need to be mutex anymore
        let best_fitness = *best_fitness.lock().unwrap();
        if mutation_area_current_fitness > best_fitness {
            debug!("we are evolving! :)\ncurrent score: {}, mutation score: {}", mutation_area_current_fitness, best_fitness);
            result_gene.add_polygon(winner_gene.get_last_polygon());

            if options.render_debug_rasters {
                result_gene.save_raster(Path::new(&format!("./tmp/evolution-{}.png", evolutions))).unwrap();
            }

            evolutions += 1;
        } else {
            debug!("mutation did not improve gene :(\ncurrent score: {}, mutation score: {}", mutation_area_current_fitness, best_fitness);
        }
    }
    result_gene.save_raster(Path::new("result.png")).unwrap();
    println!("result saved to result.png");

    Ok(())
}
