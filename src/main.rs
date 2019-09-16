mod cell;

use cell::{Cell, Genome};
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
            io::stdout().flush();
            for _ in 0..5 {
                thread::sleep(time::Duration::from_millis(500));
                print!(".");
                io::stdout().flush();
            }
            thread::sleep(time::Duration::from_millis(250));
            println!();
            Genome::from_hex_str(genome_string)
        }
        None => {
            print!("No genome given.");

            print!("Generating genome");
            io::stdout().flush();

            for _ in 0..5 {
                thread::sleep(time::Duration::from_millis(500));
                print!(".");
                io::stdout().flush();
            }

            thread::sleep(time::Duration::from_millis(250));
            println!("\nF6AAC27D1506EDBDF14D611C08DAC7EF6");
            Genome::new()
        }
    };

    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
    const TIMESTEP_COUNT: i32 = 40;
    const CELL_COUNT: usize = 48;

    let mut cells: Vec<Cell> = (0..CELL_COUNT).map(|_| rand::random::<Cell>()).collect();
    cells.iter().for_each(|c| print!("{}", c));
    println!();

    for _ in 0..TIMESTEP_COUNT {
        let mut next_cells: Vec<Cell> = Vec::with_capacity(CELL_COUNT);

        // now mutate cells so we can have our window
        let window_size = 7;
        let cap_size = 7 / 2;
        let cap_front = &cells[cells.len() - cap_size..cells.len()];
        let cap_back = &cells[0..cap_size];

        let mut iter = Vec::with_capacity(cells.len() + cap_size * 2);

        // TODO: have this handled else where.
        // We just want a get_next_state(cell);
        iter.extend_from_slice(cap_front);
        iter.extend_from_slice(&cells[..]);
        iter.extend_from_slice(cap_back);

        for window in iter.windows(window_size) {
            let state = genome.foo(&window);
            next_cells.push(Cell { state });
        }

        // update curr to next
        cells = next_cells;

        thread::sleep(SLEEP_DURATION);

        cells.iter().for_each(|c| print!("{}", c));
        println!();
    }
    println!();
    println!("Genome:{:X}", genome);
}
