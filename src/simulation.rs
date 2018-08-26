use image::DynamicImage;
use image::GenericImage;
use image_utils::{get_average_color, image_area_diff};
use individual::GImage;
use individual::Individual;
use scoped_threadpool::Pool;
use shape::{Polygon, Shapes};
use std::default::Default;
use std::path::Path;
use std::sync::{Arc, RwLock};

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
        Options {
            pop_size: 100,
            shape: "rectangle".to_owned(),
            max_iter: 200,
            num_genes: 100,
            render_debug_rasters: false,
        }
    }
}

pub fn run(target: Arc<DynamicImage>, options: Options) -> Result<()> {
    info!(
        "running simulation, shape: {}, max_generations: {}, max_genes: {}",
        options.shape, options.max_iter, options.num_genes
    );

    // let mut population = make_population(options.pop_size, target);
    let avg_color = get_average_color(target.clone());
    let (width, height) = target.dimensions();
    let mut result_gene = GImage::new(1, target.clone(), avg_color, width, height);
    let width = width as f32;
    let height = height as f32;

    let shape = match options.shape.as_str() {
        "rectangle" => Shapes::Rectangle,
        "triangle" => Shapes::Triangle,
        _ => Shapes::Rectangle,
    };

    let mut evolutions = 0;
    for i in 0..options.max_iter {
        debug!("generation {}", i);

        // generate candidate
        let new_shape = Polygon::new(shape, width, height);
        let mut mutations: Vec<GImage> = Vec::new();
        for j in 0..options.num_genes {
            let new_gene = result_gene.mutate(new_shape.clone(), j);
            mutations.push(new_gene);
        }

        let mut best_fitness = Arc::new(RwLock::new(10_000_000.0));
        let mut best_id = Arc::new(RwLock::new(0));
        let mut pool = Pool::new(4);
        pool.scoped(|scoped| {
            for gene in &mutations {
                let best_fitness = best_fitness.clone();
                let best_id = best_id.clone();

                scoped.execute(move || {
                    let fitness = gene.fitness_mutation();

                    if fitness < *best_fitness.read().unwrap() {
                        let mut w = best_fitness.write().unwrap();
                        *w = fitness;
                        let mut w = best_id.write().unwrap();
                        *w = gene.id();
                    }
                });
            }
        });

        let winner_gene = mutations
            .iter()
            .find(|ref mutation| mutation.id() == *best_id.read().unwrap())
            .ok_or(format_err!("Unexpected end of mutation list, unable to find winner gene"))?;

        debug!("we have a winner: {}", winner_gene.get_last_polygon().svg());

        let mutation_area_current_fitness = image_area_diff(
            target.clone(),
            &result_gene.as_rgba_img()?,
            winner_gene.mutation_area(),
        );

        // no need to be mutex anymore
        let best_fitness = *best_fitness.read().unwrap();
        if mutation_area_current_fitness > best_fitness {
            info!(
                "we are evolving! :) current score: {}, mutation score: {}",
                mutation_area_current_fitness, best_fitness
            );
            result_gene.add_polygon(winner_gene.get_last_polygon());

            if options.render_debug_rasters {
                let fname = format!("./tmp/evolution-{}.png", evolutions);
                debug!("saving raster of the current generation: {}", fname);
                result_gene.save_raster(Path::new(&fname))?;
            }

            evolutions += 1;
        } else {
            warn!(
                "mutation did not improve gene :(\ncurrent score: {}, mutation score: {}",
                mutation_area_current_fitness, best_fitness
            );
        }
    }
    debug!("finished.");

    result_gene.save_raster(Path::new("result.png"))?;
    info!("result saved to result.png");

    Ok(())
}
