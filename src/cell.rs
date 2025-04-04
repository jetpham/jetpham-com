use color_art::Color;

#[derive(Debug, Clone)]
pub enum Cell {
    Alive(Color),
    Dead,
}

impl Cell {
    /// Returns `true` if the cell is [`Alive`].
    ///
    /// [`Alive`]: Cell::Alive
    #[must_use]
    pub fn is_alive(&self) -> bool {
        matches!(self, Self::Alive(..))
    }
}

pub fn with_neighbors(window: Vec<&Cell>, survival: &[usize], birth: &[usize]) -> Cell {
    let center = window[4];
    let neighbors = window
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| if i == 4 { None } else { Some(cell) });
    match center {
        Cell::Alive(_) => {
            if survival.contains(&neighbors.filter(|cell| cell.is_alive()).count()) {
                center.clone()
            } else {
                Cell::Dead
            }
        }
        Cell::Dead => {
            let alive_neighbors: Vec<&Color> = neighbors
                .filter_map(|cell| match cell {
                    Cell::Alive(color) => Some(color),
                    Cell::Dead => None,
                })
                .collect();
            if birth.contains(&alive_neighbors.len()) {
                Cell::Alive(mix_colors(alive_neighbors))
            } else {
                center.clone()
            }
        }
    }
}

fn mix_colors(colors: Vec<&Color>) -> Color {
    let (sum_sin, sum_cos) = colors
        .iter()
        .map(|color| color.hue())
        .fold((0.0, 0.0), |acc, h| {
            let rad = h * std::f64::consts::PI / 180.0;
            (acc.0 + rad.sin(), acc.1 + rad.cos())
        });
    let mut avg_hue = sum_sin.atan2(sum_cos) * 180.0 / std::f64::consts::PI;
    if avg_hue < 0.0 {
        avg_hue += 360.0
    }
    Color::from_hsv(avg_hue, 1.0, 1.0).unwrap()
}
