use clap::{App, Arg};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;
use std::io::{self, Write};
use std::{thread, time};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Off = 0,
    On = 1,
}
struct Cell {
    state: State,
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        Cell {
            state: match rng.gen_bool(0.5) {
                true => State::On,
                false => State::Off,
            },
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.state {
            State::On => '🔵',
            State::Off => '🔴',
        };
        write!(f, "{}", c)
    }
}

const NUM_CELLS_PER_CONFIGURATION: usize = 7;

// TODO: have this return state
struct Genome {
    sequence: Vec<bool>,
}
impl Genome {
    const SIZE: usize = 128;

    fn new() -> Genome {
        let mut rng = rand::thread_rng();
        Genome {
            sequence: (0..Genome::SIZE).map(|_| rng.gen_bool(0.5)).collect(),
        }
    }

    fn from_hex_str(s: &str) -> Genome {
        assert_eq!(s.len(), Genome::SIZE / 4);

        let mut sequence: Vec<bool> = Vec::with_capacity(Genome::SIZE);
        for c in s.chars() {
            assert_eq!(c.is_digit(16), true);

            match c.to_digit(16) {
                Some(mut d) => {
                    while d != 0 {
                        sequence.push((d & 0b01) == 0b01);
                        d >>= 1;
                    }
                }
                None => panic!("illegal character in genome {}", c),
            }
        }
        Genome { sequence }
    }

    fn foo(&self, neighborhood: &[Cell]) -> State {
        // TODO make guarantee about size
        assert_eq!(neighborhood.len(), 7);

        let key = neighborhood.iter().fold(0, |accumulator, cell| {
            (accumulator << 1) | (cell.state as u8)
        });

        match self.sequence[key as usize] {
            true => State::On,
            false => State::Off,
        }
    }
}

impl fmt::UpperHex for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO convert this string to a cool HEX
        for chunk in self.sequence.chunks_exact(4) {
            let c = chunk.iter().fold(0, |acc, b| (acc << 1) | (*b as u8));
            write!(f, "{:X}", c)?
        }
        Ok(())
    }
}

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
        // TODO just push three
        next_cells.push(Cell { state: State::Off });
        next_cells.push(Cell { state: State::Off });
        next_cells.push(Cell { state: State::Off });

        // for earch cell
        for i in 3..CELL_COUNT - 3 {
            let window = &cells[i - 3..=i + 3];
            let state = genome.foo(&window);
            next_cells.push(Cell { state });
        }

        // calc the cells we're not handling for now
        next_cells.push(Cell { state: State::Off });
        next_cells.push(Cell { state: State::Off });
        next_cells.push(Cell { state: State::Off });

        // update curr to next
        cells = next_cells;

        thread::sleep(SLEEP_DURATION);

        cells.iter().for_each(|c| print!("{}", c));
        println!();
    }
    println!();
    println!("Genome:{:X}", genome);
}
