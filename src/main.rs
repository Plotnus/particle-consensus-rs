use clap::{App, Arg};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;
use std::io::{self, Write};
use std::{thread, time};

#[derive(Debug, PartialEq, Eq, Hash)]
enum State {
    On,
    Off,
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

    fn from_string(s: &str) -> Genome {
        assert_eq!(s.len(), Genome::SIZE / 4);

        let mut sequence: Vec<bool> = Vec::with_capacity(Genome::SIZE);
        for c in s.chars() {
            match c {
                '0' => sequence.append(&mut vec![false, false, false, false]),
                '1' => sequence.append(&mut vec![true, false, false, false]),
                '2' => sequence.append(&mut vec![false, true, false, false]),
                '3' => sequence.append(&mut vec![true, true, false, false]),
                '4' => sequence.append(&mut vec![false, false, true, false]),
                '5' => sequence.append(&mut vec![true, false, true, false]),
                '6' => sequence.append(&mut vec![false, true, true, false]),
                '7' => sequence.append(&mut vec![true, true, true, false]),
                '8' => sequence.append(&mut vec![false, false, false, true]),
                '9' => sequence.append(&mut vec![true, false, false, true]),
                'A' => sequence.append(&mut vec![false, true, false, true]),
                'B' => sequence.append(&mut vec![true, true, false, true]),
                'C' => sequence.append(&mut vec![false, false, true, true]),
                'D' => sequence.append(&mut vec![true, false, true, true]),
                'E' => sequence.append(&mut vec![false, true, true, true]),
                'F' => sequence.append(&mut vec![true, true, true, true]),
                _ => panic!("illegal character in genome {}", c),
            }
        }

        Genome { sequence }
    }

    // TODO: clean this up... too complicated
    fn foo(&self, x: &[bool; NUM_CELLS_PER_CONFIGURATION]) -> State {
        let mut index = 0;
        if x[0] {
            index |= 0b1000000;
        }
        if x[1] {
            index |= 0b0100000;
        }
        if x[2] {
            index |= 0b0010000;
        }
        if x[3] {
            index |= 0b0001000;
        }
        if x[4] {
            index |= 0b0000100;
        }
        if x[5] {
            index |= 0b0000010;
        }
        if x[6] {
            index |= 0b0000001;
        }

        if self.sequence[index] {
            State::On
        } else {
            State::Off
        }
    }
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO convert this string to a cool HEX
        for chunk in self.sequence.chunks_exact(4) {
            let c = match chunk {
                &[false, false, false, false] => '0',
                &[true, false, false, false] => '1',
                &[false, true, false, false] => '2',
                &[true, true, false, false] => '3',
                &[false, false, true, false] => '4',
                &[true, false, true, false] => '5',
                &[false, true, true, false] => '6',
                &[true, true, true, false] => '7',
                &[false, false, false, true] => '8',
                &[true, false, false, true] => '9',
                &[false, true, false, true] => 'A',
                &[true, true, false, true] => 'B',
                &[false, false, true, true] => 'C',
                &[true, false, true, true] => 'D',
                &[false, true, true, true] => 'E',
                &[true, true, true, true] => 'F',
                _ => panic!("Genome sequence is not a multiple of chunk size"),
            };
            write!(f, "{}", c)?
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
            Genome::from_string(genome_string)
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
    //process::exit(1);

    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
    const TIMESTEP_COUNT: i32 = 40;
    const CELL_COUNT: usize = 48;

    let mut cells: Vec<Cell> = (0..CELL_COUNT).map(|_| rand::random::<Cell>()).collect();

    for _ in 0..TIMESTEP_COUNT {
        let mut next_cells: Vec<Cell> = Vec::with_capacity(CELL_COUNT);
        // TODO just push three
        next_cells.push(Cell { state: State::Off });
        next_cells.push(Cell { state: State::Off });
        next_cells.push(Cell { state: State::Off });

        for i in 3..CELL_COUNT - 3 {
            let sequence = [
                cells[i - 3].state == State::On,
                cells[i - 2].state == State::On,
                cells[i - 1].state == State::On,
                cells[i].state == State::On,
                cells[i + 1].state == State::On,
                cells[i + 2].state == State::On,
                cells[i + 3].state == State::On,
            ];

            let state = genome.foo(&sequence);

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
    println!("Genome:{}", genome);
}
