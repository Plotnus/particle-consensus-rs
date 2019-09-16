use crate::cell::{Cell, Genome};

extern crate rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone)]
pub struct Organism {
    // collection of cells
    cells: Vec<Cell>,
    genome: Genome,
}

impl Organism {
    const NUM_CELLS: usize = 48;
    // TODO: add way to specifiy initial configuration
    pub fn from_genome(genome: &Genome) -> Organism {
        // how to spawn each cell?

        // TODO: have version where rng is passed in...
        let mut rng = rand::thread_rng();
        let cells = std::iter::repeat_with(|| Cell { state: rng.gen() })
            .take(Organism::NUM_CELLS)
            .collect();

        Organism {
            cells: cells,
            genome: genome.clone(),
        }
    }

    // To move into the future, we must consume our past
    pub fn tick(mut self) -> Organism {
        let window_size = 7;
        let cap_size = 7 / 2;
        let cap_front = &self.cells[(self.cells.len() - cap_size)..self.cells.len()];
        let cap_back = &self.cells[0..cap_size];

        let mut iter = Vec::with_capacity(self.cells.len() + cap_size * 2);
        iter.extend_from_slice(cap_front);
        iter.extend_from_slice(&self.cells[..]);
        iter.extend_from_slice(cap_back);

        for (i, window) in iter.windows(window_size).enumerate() {
            let state = self.genome.foo(&window);
            self.cells[i].state = state;
        }

        self
    }
}
impl std::fmt::Display for Organism {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in self.cells.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}
