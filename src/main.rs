use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;
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
            state: if rng.gen_bool(0.5) {
                State::On
            } else {
                State::Off
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
const CONFIGURATION_COUNT: i32 = 128; // 2 ^ NUM_CELLS_PER_CONFIGURATION

struct Genome {
    sequence: Vec<bool>,
}
impl Genome {
    fn new() -> Genome {
        let mut rng = rand::thread_rng();
        Genome {
            sequence: (0..CONFIGURATION_COUNT)
                .map(|_| rng.gen_bool(0.5))
                .collect(),
        }
    }
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
        for b in self.sequence.iter() {
            write!(f, "{}", if *b { '1' } else { '0' })?
        }
        Ok(())
    }
}

fn main() {
    const SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
    const TIMESTEP_COUNT: i32 = 40;
    const CELL_COUNT: usize = 48;

    let genome: Genome = Genome::new();

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
        print!("\n");
    }
    print!("Genome:{}", genome);
}
