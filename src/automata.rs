use crate::cell::Cell;

use color_art::Color;
use grid::Grid;
use log::info;
use ratzilla::ratatui::layout::Rect;
use ratzilla::ratatui::widgets::Widget;
use ratzilla::ratatui::widgets::canvas::{Canvas, Points};
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct Automata {
    grid: Grid<Cell>,
    birth: Vec<usize>,
    survival: Vec<usize>,
}

fn random_color() -> Color {
    Color::from_hsv(fastrand::f64() * 360.0, 1.0, 1.0).unwrap()
}

fn rat_color(color: color_art::Color) -> ratzilla::ratatui::prelude::Color {
    ratzilla::ratatui::style::Color::Rgb(color.red(), color.green(), color.blue())
}

impl Automata {
    fn colors<'a>(&'a self) -> Box<dyn Iterator<Item = ((usize, usize), Color)> + 'a> {
        Box::new(self.grid.indexed_iter().filter_map(|(x, y)| match y {
            Cell::Alive(color) => Some((x, *color)),
            Cell::Dead => None,
        }))
    }

    // fn draw(&mut self, draw_row: usize, draw_col: usize) {
    //     if let Some(state) = self.grid.get_mut(draw_row, draw_col) {
    //         *state = Cell::Alive(random_color());
    //         info!("Cell at ({}, {}) toggled", draw_row, draw_col);
    //     } else {
    //         info!(
    //             "Draw missed: coordinates ({}, {}) out of bounds",
    //             draw_row, draw_col
    //         );
    //     }
    // }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<&Cell> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .fold(Vec::new(), |mut neighbors, (moore_row, moore_col)| {
            let row = match add_and_cast(row, *moore_row) {
                Some(row) => row,
                None => return neighbors,
            };
            let col = match add_and_cast(col, *moore_col) {
                Some(col) => col,
                None => return neighbors,
            };
            if let Some(neighbor) = self.grid.get(row, col) {
                neighbors.push(neighbor);
            }
            neighbors
        })
    }

    pub fn life_canvas(&self, area: Rect) -> impl Widget + '_ {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height);
        Canvas::default()
            .marker(ratzilla::ratatui::symbols::Marker::Block)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                self.colors().for_each(|((row, col), color)| {
                    ctx.draw(&Points {
                        coords: &[(col as f64, row as f64)],
                        color: rat_color(color),
                    });
                });
            })
    }

    pub fn new(width: usize, height: usize, birth: Vec<usize>, survival: Vec<usize>) -> Self {
        let random_grid = Grid::from_vec(
            (0..width * height)
                .map(|_| -> Cell {
                    match fastrand::bool() {
                        true => Cell::Alive(random_color()),
                        false => Cell::Dead,
                    }
                })
                .collect(),
            width,
        );
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
        }
    }

    pub fn step(&mut self) {
        let grid = &self.grid;
        let mut new_grid = grid.clone();

        new_grid.indexed_iter_mut().for_each(|((row, col), cell)| {
            let neighbors = self.get_neighbors(row, col);
            *cell = cell.from_neighbors(neighbors, &self.survival, &self.birth);
        });
        self.grid = new_grid
    }
}

fn add_and_cast(a: usize, b: i8) -> Option<usize> {
    let b_as_isize = b as isize;
    a.checked_add_signed(b_as_isize)
}
