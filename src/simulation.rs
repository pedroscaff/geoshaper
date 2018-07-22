use image::Rgba;
use image::DynamicImage;
use individual::GImage;
use std::path::Path;
use std::sync::Arc;
use image_utils::{get_average_color, image_area_diff};
use image::GenericImage;
use individual::Individual;
use shape::{Polygon, Shapes};

use error::{Result, ResultExt};

#[derive(Debug)]
pub struct Options {
    pub pop_size: u32,
    pub shape: String
}

pub fn run(target: Arc<DynamicImage>, options: Options) -> Result<()> {
    // let mut population = make_population(options.pop_size, target);
    let max_iter = 200;
    let num_genes = 100;
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
    for i in 0..max_iter {
        // generate candidate
        let new_shape = Polygon::new(shape, width, height);
        let mut mutations : Vec<GImage> = Vec::new();
        for j in 0..num_genes {
            let new_gene = result_gene.mutate(new_shape.clone(), j);
            mutations.push(new_gene);
        } 

        let mut best_fitness = 10_000_000.0;
        let mut best_index = 0;
        for (index, ref gene) in mutations.iter().enumerate() {
            let fitness = gene.fitness_mutation();
            if fitness < best_fitness {
                best_fitness = fitness;
                best_index = index;
            }
        }
        let winner_gene = &mutations[best_index];
        println!("we have a winner: {}", winner_gene.get_last_polygon().svg());

        let mutation_area_current_fitness = image_area_diff(
            target.clone(),
            &result_gene.as_rgba_img().unwrap(),
            winner_gene.mutation_area()
        );
        if mutation_area_current_fitness > best_fitness {
            println!("we are evolving! :)\ncurrent score: {}, mutation score: {}", mutation_area_current_fitness, best_fitness);
            result_gene.add_polygon(winner_gene.get_last_polygon());
            result_gene.save_raster(Path::new(&format!("./tmp/evolution-{}.png", evolutions))).unwrap();
            evolutions += 1;
        } else {
            println!("mutation did not improve gene :(\ncurrent score: {}, mutation score: {}", mutation_area_current_fitness, best_fitness);
        }
    }
    result_gene.save_raster(Path::new("result.png")).unwrap();

    Ok(())
}
