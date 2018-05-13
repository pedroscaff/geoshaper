use darwin_rs::{SimulationBuilder, PopulationBuilder};
use image::DynamicImage;
use individual::GImage;
// use std::io::Error;
use std::sync::Arc;
use image_utils::get_average_color;

#[derive(Debug)]
pub struct Options {
    pub pop_size: u32,
}

fn make_population(size: u32, target: Arc<DynamicImage>) -> Vec<GImage> {
    let mut pop_array: Vec<GImage> = Vec::with_capacity(size as usize);
    let avg_color = get_average_color(target.clone());
    for i in 0..size {
        pop_array.push(GImage::new(i, target.clone(), avg_color));
    }
    pop_array
}

pub fn run(target: Arc<DynamicImage>, options: Options) {
    let my_pop = make_population(options.pop_size, target);

    let population1 = PopulationBuilder::<GImage>::new()
        .set_id(1)
        .initial_population(&my_pop)
        .increasing_exp_mutation_rate(1.03)
        .reset_limit_increment(100)
        .reset_limit_start(100)
        .reset_limit_end(1000)
        .finalize()
        .unwrap();


    let population2 = PopulationBuilder::<GImage>::new()
        .set_id(2)
        .initial_population(&my_pop)
        .increasing_exp_mutation_rate(1.04)
        .reset_limit_increment(200)
        .reset_limit_start(100)
        .reset_limit_end(2000)
        .finalize()
        .unwrap();

    let my_builder = SimulationBuilder::<GImage>::new()
        .factor(0.34)
        .threads(2)
        .add_population(population1)
        .add_population(population2)
        .finalize();

    match my_builder {
        Err(_) => println!("more than 10 iteratons needed"),
        Ok(mut my_simulation) => {
            my_simulation.run();

            my_simulation.simulation_result.fittest[0].individual.save("fit.png").unwrap();

            println!("total run time: {} ms", my_simulation.total_time_in_ms);
            println!(
                "improvement factor: {}",
                my_simulation.simulation_result.improvement_factor
            );
            println!(
                "number of iterations: {}",
                my_simulation.simulation_result.iteration_counter
            );

            my_simulation.print_fitness();
        }
    }

    // unimplemented!()

}
