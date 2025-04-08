use crate::cell::{Cell, with_neighbors};
use web_time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use log::info;
use ndarray::{self, Array, Array2, Zip};
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct Automata {
    grid: Array2<Cell>,
    birth: Vec<usize>,
    survival: Vec<usize>,
    last_tick: Instant,
    glider_last_tick: Instant,
}

const TIME_STEP: Duration = Duration::from_millis(100);
const GLIDER_TIME_STEP: Duration = Duration::from_millis(500);

fn random_color() -> color_art::Color {
    color_art::Color::from_hsv(fastrand::f64() * 360.0, 1.0, 1.0).unwrap()
}

fn rat_color(color: color_art::Color) -> ratzilla::ratatui::prelude::Color {
    ratzilla::ratatui::style::Color::Rgb(color.red(), color.green(), color.blue())
}

impl Automata {
    pub(crate) fn rat_colors(&self) -> Vec<Option<ratzilla::ratatui::style::Color>> {
        self.grid
            .iter()
            .map(|cell| match cell {
                Cell::Alive(color) => Some(rat_color(*color)),
                Cell::Dead => None,
            })
            .collect()
    }

    pub fn new(width: usize, height: usize, birth: Vec<usize>, survival: Vec<usize>) -> Self {
        fastrand::seed(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::new(1, 0))
                .as_millis()
                .try_into()
                .unwrap_or_default(),
        );

        let random_grid = Array::from_shape_simple_fn((height, width), || match fastrand::bool() {
            true => Cell::Alive(random_color()),
            false => Cell::Dead,
        });
        info!(
            "grid initialized\nsize:({}, {})\nbirth: {:?}\nsurvival: {:?}\ndensity: {}",
            width,
            height,
            birth,
            survival,
            random_grid
                .iter()
                .filter(|x| matches!(x, Cell::Alive(_)))
                .count()
        );
        Automata {
            grid: random_grid,
            birth,
            survival,
            last_tick: Instant::now(),
            glider_last_tick: Instant::now(),
        }
    }

    fn padded_mirror_grid(&self, pad: usize) -> Array2<Cell> {
        let (rows, cols) = self.grid.dim();

        Array2::from_shape_fn((rows + 2 * pad, cols + 2 * pad), |(i, j)| {
            let mirrored_i = if i < pad {
                pad - i - 1
            } else if i >= rows + pad {
                2 * rows + pad - i - 1
            } else {
                i - pad
            };

            let mirrored_j = if j < pad {
                pad - j - 1
            } else if j >= cols + pad {
                2 * cols + pad - j - 1
            } else {
                j - pad
            };

            self.grid[(mirrored_i, mirrored_j)].clone()
        })
    }

    pub fn update(&mut self) {
        if self.last_tick.elapsed() >= TIME_STEP {
            self.last_tick = Instant::now();
            self.step();
        }
        if self.glider_last_tick.elapsed() >= GLIDER_TIME_STEP {
            self.glider_last_tick = Instant::now();
            self.spawn_glider();
        }
    }

    fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        Zip::from(&mut new_grid)
            .and(self.padded_mirror_grid(1).windows((3, 3)))
            .for_each(|new_cell, window| {
                let window_slice: Vec<&Cell> = window.iter().collect();
                *new_cell = with_neighbors(window_slice, &self.survival, &self.birth);
            });
        self.grid = new_grid
    }

    fn spawn_glider(&mut self) {
        // empty area for a glider to spawn in
        const GLIDER_AREA: usize = 5;
        const GLIDER_PATTERNS: [[[bool; 3]; 3]; 4] = [
            [
                [false, false, true],
                [true, false, true],
                [false, true, true],
            ],
            [
                [false, true, false],
                [true, false, false],
                [true, true, true],
            ],
            [
                [true, true, false],
                [true, false, true],
                [true, false, false],
            ],
            [
                [true, true, true],
                [false, false, true],
                [false, true, false],
            ],
        ];

        let (rows, cols) = self.grid.dim();

        // Get all window positions for GLIDER_AREA^2
        let mut positions: Vec<(usize, usize)> = (0..=rows - GLIDER_AREA)
            .flat_map(|i| (0..=cols - GLIDER_AREA).map(move |j| (i, j)))
            .collect();

        fastrand::shuffle(&mut positions);

        for (start_row, start_col) in positions {
            let is_all_dead = (0..GLIDER_AREA).all(|i| {
                (0..GLIDER_AREA)
                    .all(|j| matches!(self.grid[(start_row + i, start_col + j)], Cell::Dead))
            });

            if is_all_dead {
                // Choose a random pattern
                let glider_pattern = &GLIDER_PATTERNS[fastrand::usize(..4)];

                let center_row = start_row + GLIDER_AREA / 2;
                let center_col = start_col + GLIDER_AREA / 2;

                // Fill center with a random glider orientation
                (0..3).for_each(|i| {
                    for j in 0..3 {
                        if glider_pattern[i][j] {
                            self.grid[(center_row + i - 1, center_col + j - 1)] =
                                Cell::Alive(random_color());
                        }
                    }
                });

                return;
            }
        }

        info!("No suitable location found for spawning a glider.");
    }

    pub(crate) fn resize(&mut self, new_width: usize, new_height: usize) {
        *self = Automata::new(
            new_width,
            new_height,
            self.birth.clone(),
            self.survival.clone(),
        );
    }
}
