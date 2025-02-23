use color_art::Color;

#[derive(Debug, Clone)]
pub enum Cell {
    Alive(Color),
    Dead,
}

impl Cell {
    pub fn from_neighbors(
        &self,
        neighbors: Vec<&Cell>,
        survival: &Vec<usize>,
        birth: &Vec<usize>,
    ) -> Cell {
        let neighbor_count = neighbors
            .iter()
            .filter(|cell| match cell {
                Cell::Alive(_) => true,
                Cell::Dead => false,
            })
            .count();
        match self {
            Cell::Alive(_) => {
                if survival.contains(&neighbor_count) {
                    self.clone()
                } else {
                    Cell::Dead
                }
            }
            Cell::Dead => {
                if birth.contains(&neighbor_count) {
                    let neighbor_colors: Vec<&Color> = neighbors
                        .iter()
                        .filter_map(|cell| match cell {
                            Cell::Alive(color) => Some(color),
                            Cell::Dead => None,
                        })
                        .collect();
                    Cell::Alive(mix_colors(neighbor_colors))
                } else {
                    self.clone()
                }
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
