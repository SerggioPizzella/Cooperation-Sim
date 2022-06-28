use std::{cmp::Ordering, vec};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

fn main() {
    let width = 20;
    let height = 20;
    let mut playing_field = PlayingField::new(width, height);

    let aplha = 10_f32;

    for _ in 0..200 {
        print_playing_field(&playing_field);
        playing_field.step(aplha);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strategy {
    Helping,
    Leeching,
}

impl Distribution<Strategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Strategy {
        match rng.gen_bool(0.5) {
            true => Strategy::Helping,
            false => Strategy::Leeching,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Patch {
    strategy: Strategy,
    score: f32,
}

impl Patch {
    fn new(strategy: Strategy) -> Patch {
        Patch {
            strategy,
            score: 0f32,
        }
    }

    // fn flip(&mut self) {
    //     self.strategy = match self.strategy {
    //         Strategy::Helping => Strategy::Leeching,
    //         Strategy::Leeching => Strategy::Helping,
    //     }
    // }
}

#[derive(Debug)]
struct PlayingField {
    width: usize,
    height: usize,
    patches: Vec<Vec<Patch>>,
}

impl PlayingField {
    fn new(width: usize, height: usize) -> PlayingField {
        let mut rng = rand::thread_rng();
        let mut patches = vec![Vec::with_capacity(height); width as usize];

        for x in 0..width {
            for _ in 0..height {
                patches.get_mut(x).unwrap().push(Patch::new(rng.gen()));
            }
        }

        PlayingField {
            width,
            height,
            patches,
        }
    }

    fn step(&mut self, alpha: f32) {
        let patches = self.patches.to_owned();
        let w = self.width as isize;
        let h = self.height as isize;

        // Calculate the score for all patches
        let mut neighbours = Vec::with_capacity(8);
        for x in 0..self.patches.len() as isize{
            for y in 0..self.patches[x as usize].len() as isize{
                neighbours.push(&patches[((((x - 1) % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize]);
                neighbours.push(&patches[((((x - 1) % w) + w) % w) as usize][(((y % h) + h) % h) as usize]);
                neighbours.push(&patches[((((x - 1) % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize]);
                neighbours.push(&patches[(((x % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize]);
                neighbours.push(&patches[(((x % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize]);
                neighbours.push(&patches[((((x + 1) % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize]);
                neighbours.push(&patches[((((x + 1) % w) + w) % w) as usize][(((y % h) + h) % h) as usize]);
                neighbours.push(&patches[((((x + 1) % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize]);

                self.patches[x as usize][y as usize].score = calculate_score(&self.patches[x as usize][y as usize], &neighbours, alpha);
            }
        }

        // Set new strategy for each patch
        for x in 0..self.patches.len() as isize {
            for y in 0..self.patches[x as usize].len() as isize {
                let winning_patch = {
                    let neighbours = [
                        &patches[((((x - 1) % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize],
                        &patches[((((x - 1) % w) + w) % w) as usize][(((y % h) + h) % h) as usize],
                        &patches[((((x - 1) % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize],
                        &patches[(((x % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize],
                        &patches[(((x % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize],
                        &patches[((((x + 1) % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize],
                        &patches[((((x + 1) % w) + w) % w) as usize][(((y % h) + h) % h) as usize],
                        &patches[((((x + 1) % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize]
                    ];

                    neighbours
                        .iter()
                        .max_by(|a, b| {
                            if (a.score - b.score).abs() < f32::EPSILON {
                                Ordering::Equal
                            } else if a.score > b.score {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        })
                        .unwrap()
                        .to_owned()
                };

                self.patches[x as usize][y as usize].strategy = winning_patch.strategy;
            }
        }
    }
}

fn calculate_score(patch: &Patch, neighbours: &Vec<&Patch>, alpha: f32) -> f32 {
    match patch.strategy {
        Strategy::Helping => {
            neighbours
                .iter()
                .filter(|&p| p.strategy == Strategy::Helping)
                .count() as f32
        }
        Strategy::Leeching => {
            neighbours
                .iter()
                .filter(|&p| p.strategy == Strategy::Helping)
                .count() as f32
                * alpha
        }
    }
}

fn print_playing_field(playing_field: &PlayingField) {
    print!("\x1B[2J\x1B[1;1H");
    for patch_row in &playing_field.patches {
        for patch in patch_row {
            match patch.strategy {
                Strategy::Helping => print!("✅"),
                Strategy::Leeching  => print!("❌")
            }
        }
        println!();
    }
}
