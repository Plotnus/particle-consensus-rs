use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

const NUM_CELLS_PER_CONFIGURATION: usize = 7;

#[derive(Clone)]
pub struct Cell {
    // TODO make state private
    pub state: State,
}

// TODO: make state private
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    Off = 0,
    On = 1,
}
impl State {
    pub fn from_bool(b: bool) -> State {
        if b {
            State::On
        } else {
            State::Off
        }
    }
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
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.state {
            State::On => '🔵',
            State::Off => '🔴',
        };
        write!(f, "{}", c)
    }
}

// TODO: split into Genome, Society,
pub struct Genome {
    sequence: Vec<bool>,
}
impl Genome {
    const SIZE: usize = 128;

    pub fn new() -> Genome {
        let mut rng = rand::thread_rng();
        Genome {
            sequence: (0..Genome::SIZE).map(|_| rng.gen_bool(0.5)).collect(),
        }
    }

    pub fn from_hex_str(s: &str) -> Genome {
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

    // TODO: rename or restructure
    pub fn foo(&self, neighborhood: &[Cell]) -> State {
        // TODO make guarantee about size
        assert_eq!(neighborhood.len(), 7);

        let key = neighborhood.iter().fold(0, |accumulator, cell| {
            (accumulator << 1) | (cell.state as u8)
        });

        State::from_bool(self.sequence[key as usize])
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
