use std::{cmp::Ordering, vec, thread::sleep, time::Duration};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let size: usize = arguments.next().unwrap().parse().unwrap();
    let iterations: usize = arguments.next().unwrap().parse().unwrap();
    let mut playing_field = PlayingField::new(size, size);

    let aplha = 1.3_f32;

    for _ in 0..iterations {
        print_playing_field(&playing_field).expect("idk man");
        playing_field.step(aplha);
        sleep(Duration::from_millis(500));
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

    fn with_patches(patches: Vec<Vec<Patch>>) -> PlayingField {
        PlayingField { width: patches.len(), height: patches[0].len(), patches }
    }

    fn step(&mut self, alpha: f32) {
        let patches = self.patches.to_owned();
        let w = self.width as isize;
        let h = self.height as isize;

        // Calculate the score for all patches
        for x in 0..self.patches.len() as isize{
            for y in 0..self.patches[x as usize].len() as isize{
                let mut neighbours = Vec::with_capacity(8);
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
                        &self.patches[((((x - 1) % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize],
                        &self.patches[((((x - 1) % w) + w) % w) as usize][(((y % h) + h) % h) as usize],
                        &self.patches[((((x - 1) % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize],
                        &self.patches[(((x % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize],
                        &self.patches[(((x % w) + w) % w) as usize][(((y % h) + h) % h) as usize],
                        &self.patches[(((x % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize],
                        &self.patches[((((x + 1) % w) + w) % w) as usize][((((y - 1) % h) + h) % h) as usize],
                        &self.patches[((((x + 1) % w) + w) % w) as usize][(((y % h) + h) % h) as usize],
                        &self.patches[((((x + 1) % w) + w) % w) as usize][((((y + 1) % h) + h) % h) as usize]
                    ];

                    neighbours
                        .iter()
                        .max_by(|a, b| {
                            if a.score > b.score {
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

fn print_playing_field(playing_field: &PlayingField) -> std::io::Result<()> {
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    write!(lock,"\x1B[2J\x1B[1;1H")?;
    for patch_row in &playing_field.patches {
        for patch in patch_row {
            match patch.strategy {
                Strategy::Helping => write!(lock, "🟢")?,
                Strategy::Leeching  => write!(lock, "🟡")?
            };
        }

        // write!(lock, "  ")?;
        // for patch in patch_row {
        //     write!(lock, "{},", patch.score)?;
        // }
        writeln!(lock)?;
    }
    writeln!(lock)?;

    Ok(())
}

#[test]
fn single_leeching_patch() {
    let alpha = 0_f32;
    let patches = vec![vec![Patch::new(Strategy::Leeching); 1]; 1];
    let mut playing_field = PlayingField::with_patches(patches);

    assert_eq!(playing_field.patches[0][0].strategy, Strategy::Leeching);

    for _ in 0..10 {
        playing_field.step(alpha);
        assert_eq!(playing_field.patches[0][0].strategy, Strategy::Leeching);
    }
}

#[test]
fn single_helping_patch() {
    let alpha = 0_f32;
    let patches = vec![vec![Patch::new(Strategy::Helping); 1]; 1];
    let mut playing_field = PlayingField::with_patches(patches);

    assert_eq!(playing_field.patches[0][0].strategy, Strategy::Helping);

    for _ in 0..10 {
        playing_field.step(alpha);
        assert_eq!(playing_field.patches[0][0].strategy, Strategy::Helping);
    }
}

#[test]
fn single_step_small_grid() {
    let alpha = 0_f32;
    let patches = vec![vec![Patch::new(Strategy::Helping); 3]; 3];
    let mut playing_field = PlayingField::with_patches(patches.to_owned());

    playing_field.step(alpha);

    for patch_row in playing_field.patches {
        for patch in patch_row {
            assert_eq!(patch.strategy, Strategy::Helping);
            assert_eq!(patch.score, 8_f32);
        }
    }
}