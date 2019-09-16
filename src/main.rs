mod cell;
mod organism;
use cell::Genome;
use organism::Organism;

use clap::{App, Arg};
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let matches = App::new("My Cellular Automaton")
        .author("Jonathan P. <plotnus@gmail.com>")
        .version("0.0.1")
        .about("A classification cellular automaton.")
        .arg(
            Arg::with_name("genome")
                .required(false)
                .short("g")
                .long("genome")
                .takes_value(true),
        )
        .get_matches();

    let genome = match matches.value_of("genome") {
        Some(genome_string) => {
            print!("Cloning genome");
            io::stdout().flush().unwrap();
            for _ in 0..5 {
                thread::sleep(time::Duration::from_millis(500));
                print!(".");
                io::stdout().flush().unwrap();
            }
            thread::sleep(time::Duration::from_millis(250));
            println!();
            Genome::from_hex_str(genome_string)
        }
        None => {
            print!("No genome given.");

            print!("Generating genome");
            io::stdout().flush().unwrap();

            for _ in 0..5 {
                thread::sleep(time::Duration::from_millis(500));
                print!(".");
                io::stdout().flush().unwrap();
            }

            thread::sleep(time::Duration::from_millis(250));
            println!("\nF6AAC27D1506EDBDF14D611C08DAC7EF6");
            Genome::new()
        }
    };

    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
    const TIMESTEP_COUNT: usize = 40;

    let mut organism = Organism::from_genome(&genome);
    println!("{}", organism);

    for _ in 0..TIMESTEP_COUNT {
        organism = organism.tick();
        thread::sleep(SLEEP_DURATION);
        println!("{}", organism);
    }
    println!("Genome:{:X}", genome);
}
