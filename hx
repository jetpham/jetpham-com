Modified regular file Cargo.lock:
    ...
   9    9: checksum = "683d7910e743518b0e34f1186f92494becacb047c7b6bf616c96772180fef923"
  10   10: 
  11   11: [[package]]
       12: name = "autocfg"
       13: version = "1.4.0"
       14: source = "registry+https://github.com/rust-lang/crates.io-index"
       15: checksum = "ace50bade8e6234aa140d9a2f552bbee1db4d353f69b8217bc503490fc1a9f26"
       16: 
       17: [[package]]
  12   18: name = "bitflags"
  13   19: version = "2.9.0"
  14   20: source = "registry+https://github.com/rust-lang/crates.io-index"
    ...
 165  171: ]
 166  172: 
 167  173: [[package]]
 168     : name = "grid"
 169     : version = "0.16.1"
 170     : source = "registry+https://github.com/rust-lang/crates.io-index"
 171     : checksum = "fb6ae361963ea5fe52038156ea1729f3b4e4ccc0711c362ab2b2d2c0a259e7c3"
 172     : 
 173     : [[package]]
 174  174: name = "hashbrown"
 175  175: version = "0.15.2"
 176  176: source = "registry+https://github.com/rust-lang/crates.io-index"
    ...
 234  234:  "color-art",
 235  235:  "fastrand",
 236  236:  "getrandom",
 237     :  "grid",
 238  237:  "log",
      238:  "ndarray",
 239  239:  "ratzilla",
 240  240:  "textwrap",
 241  241:  "wasm-log",
      242:  "web-time",
 242  243: ]
 243  244: 
 244  245: [[package]]
    ...
 279  280: ]
 280  281: 
 281  282: [[package]]
      283: name = "matrixmultiply"
      284: version = "0.3.9"
      285: source = "registry+https://github.com/rust-lang/crates.io-index"
      286: checksum = "9380b911e3e96d10c1f415da0876389aaf1b56759054eeb0de7df940c456ba1a"
      287: dependencies = [
      288:  "autocfg",
      289:  "rawpointer",
      290: ]
      291: 
      292: [[package]]
      293: name = "ndarray"
      294: version = "0.16.1"
      295: source = "registry+https://github.com/rust-lang/crates.io-index"
      296: checksum = "882ed72dce9365842bf196bdeedf5055305f11fc8c03dee7bb0194a6cad34841"
      297: dependencies = [
      298:  "matrixmultiply",
      299:  "num-complex",
      300:  "num-integer",
      301:  "num-traits",
      302:  "portable-atomic",
      303:  "portable-atomic-util",
      304:  "rawpointer",
      305: ]
      306: 
      307: [[package]]
      308: name = "num-complex"
      309: version = "0.4.6"
      310: source = "registry+https://github.com/rust-lang/crates.io-index"
      311: checksum = "73f88a1307638156682bada9d7604135552957b7818057dcef22705b4d509495"
      312: dependencies = [
      313:  "num-traits",
      314: ]
      315: 
      316: [[package]]
 282  317: name = "num-conv"
 283  318: version = "0.1.0"
 284  319: source = "registry+https://github.com/rust-lang/crates.io-index"
 285  320: checksum = "51d515d32fb182ee37cda2ccdcb92950d6a3c2893aa280e540671c2cd0f3b1d9"
 286  321: 
 287  322: [[package]]
      323: name = "num-integer"
      324: version = "0.1.46"
      325: source = "registry+https://github.com/rust-lang/crates.io-index"
      326: checksum = "7969661fd2958a5cb096e56c8e1ad0444ac2bbcd0061bd28660485a44879858f"
      327: dependencies = [
      328:  "num-traits",
      329: ]
      330: 
      331: [[package]]
      332: name = "num-traits"
      333: version = "0.2.19"
      334: source = "registry+https://github.com/rust-lang/crates.io-index"
      335: checksum = "071dfc062690e90b734c0b2273ce72ad0ffa95f0c74596bc250dcfd960262841"
      336: dependencies = [
      337:  "autocfg",
      338: ]
      339: 
      340: [[package]]
 288  341: name = "num_threads"
 289  342: version = "0.1.7"
 290  343: source = "registry+https://github.com/rust-lang/crates.io-index"
    ...
 306  359: checksum = "57c0d7b74b563b49d38dae00a0c37d4d6de9b432382b2892f0574ddcae73fd0a"
 307  360: 
 308  361: [[package]]
      362: name = "portable-atomic"
      363: version = "1.11.0"
      364: source = "registry+https://github.com/rust-lang/crates.io-index"
      365: checksum = "350e9b48cbc6b0e028b0473b114454c6316e57336ee184ceab6e53f72c178b3e"
      366: 
      367: [[package]]
      368: name = "portable-atomic-util"
      369: version = "0.2.4"
      370: source = "registry+https://github.com/rust-lang/crates.io-index"
      371: checksum = "d8a2f0d8d040d7848a709caf78912debcc3f33ee4b3cac47d73d1e1069e83507"
      372: dependencies = [
      373:  "portable-atomic",
      374: ]
      375: 
      376: [[package]]
 309  377: name = "powerfmt"
 310  378: version = "0.2.0"
 311  379: source = "registry+https://github.com/rust-lang/crates.io-index"
    ...
 371  439: [[package]]
 372  440: name = "ratatui"
 373  441: version = "0.29.0"
 374     : source = "registry+https://github.com/rust-lang/crates.io-index"
 375     : checksum = "eabd94c2f37801c20583fc49dd5cd6b0ba68c716787c2dd6ed18571e1e63117b"
 376  442: dependencies = [
 377  443:  "bitflags",
 378  444:  "cassowary",
    ...
 391  457: 
 392  458: [[package]]
 393  459: name = "ratzilla"
 394  460: version = "0.0.45"
 395     : source = "registry+https://github.com/rust-lang/crates.io-index"
 396     : checksum = "60133b30134c635ec536d669c5cef4849fd4c53bd26f0d03716a7dbd30e680fd"
 397  461: dependencies = [
 398  462:  "console_error_panic_hook",
 399  463:  "ratatui",
    ...
 402  466: ]
 403  467: 
 404  468: [[package]]
      469: name = "rawpointer"
      470: version = "0.2.1"
      471: source = "registry+https://github.com/rust-lang/crates.io-index"
      472: checksum = "60a357793950651c4ed0f3f52338f53b2f809f32d83a07f72909fa13e4c6c1e3"
      473: 
      474: [[package]]
 405  475: name = "rustversion"
 406  476: version = "1.0.20"
 407  477: source = "registry+https://github.com/rust-lang/crates.io-index"
    ...
 683  753: ]
 684  754: 
 685  755: [[package]]
      756: name = "web-time"
      757: version = "1.1.0"
      758: source = "registry+https://github.com/rust-lang/crates.io-index"
      759: checksum = "5a6580f308b1fad9207618087a65c04e7a10bc77e02c8e84e9b00dd4b12fa0bb"
      760: dependencies = [
      761:  "js-sys",
      762:  "wasm-bindgen",
      763: ]
      764: 
      765: [[package]]
 686  766: name = "zerocopy"
 687  767: version = "0.8.24"
 688  768: source = "registry+https://github.com/rust-lang/crates.io-index"
    ...
Modified regular file Cargo.toml:
    ...
   5    5: 
   6    6: [dependencies]
   7    7: color-art = "0.3.9"
   8     : grid = "0.16.1"
   9    8: log = "0.4.26"
  10    9: ratzilla = { path = "../ratzilla/", version = "0.0.45"}
  11   10: wasm-log = "0.3.1"
  12   11: fastrand = "2.3.0"
  13   12: getrandom = { version = "0.2.15", features = ["js"] }
  14   13: textwrap = "0.16.2"
       14: web-time = "1.1.0"
       15: ndarray = "0.16.1"
Added regular file hx:
    (empty)
Modified regular file src/automata.rs:
   1    1: use crate::cell::{Cell, with_neighbors};
   1    2: use web_time::{Duration, Instant, SystemTime, UNIX_EPOCH};
   2    3: 
   3     : use grid::Grid;
   4    4: use log::info;
        5: use ndarray::{self, Array, Array2, Zip};
   5    6: use std::fmt::Debug;
   6    7: use std::vec::Vec;
   7    8: 
   8    9: #[derive(Debug)]
   9   10: pub struct Automata {
  10   11:     grid: GridArray2<Cell>,
  11   12:     birth: Vec<usize>,
  12   13:     survival: Vec<usize>,
       14:     last_tick: Instant,
       15:     glider_last_tick: Instant,
  13   16: }
  14   17: 
       18: const TIME_STEP: Duration = Duration::from_millis(100);
       19: const GLIDER_TIME_STEP: Duration = Duration::from_millis(500);
       20: 
  15   21: fn random_color() -> color_art::Color {
  16   22:     color_art::Color::from_hsv(fastrand::f64() * 360.0, 1.0, 1.0).unwrap()
  17   23: }
    ...
  26   32:             .iter()
  27   33:             .map(|cell| match cell {
  28   34:                 Cell::Alive(color) => Some(rat_color(*color)),
       35:                 Cell::Growing((color, growth)) => Some(rat_color(color.fade_in(*growth))),
  29   36:                 Cell::Dead => None,
  30   37:             })
  31   38:             .collect()
  32   39:     }
  33   40: 
  34     :     // fn draw(&mut self, draw_row: usize, draw_col: usize) {
  35     :     //     if let Some(state) = self.grid.get_mut(draw_row, draw_col) {
  36     :     //         *state = Cell::Alive(random_color());
  37     :     //         info!("Cell at ({}, {}) toggled", draw_row, draw_col);
  38     :     //     } else {
  39     :     //         info!(
  40     :     //             "Draw missed: coordinates ({}, {}) out of bounds",
  41     :     //             draw_row, draw_col
  42     :     //         );
  43     :     //     }
  44     :     // }
  45     : 
  46     :     fn get_neighbors(&self, row: usize, col: usize) -> Vec<&Cell> {
  47     :         [
  48     :             (-1, -1),
  49     :             (-1, 0),
  50     :             (-1, 1),
  51     :             (0, -1),
  52     :             (0, 1),
  53     :             (1, -1),
  54     :             (1, 0),
  55     :             (1, 1),
  56     :         ]
  57     :         .iter()
  58     :         .fold(Vec::new(), |mut neighbors, (moore_row, moore_col)| {
  59     :             let row = match add_and_cast(row, *moore_row) {
  60     :                 Some(row) => row,
  61     :                 None => return neighbors,
  62     :             };
  63     :             let col = match add_and_cast(col, *moore_col) {
  64     :                 Some(col) => col,
  65     :                 None => return neighbors,
  66     :             };
  67     :             if let Some(neighbor) = self.grid.get(row, col) {
  68     :                 neighbors.push(neighbor);
  69     :             }
  70     :             neighbors
  71     :         })
  72     :     }
  73     : 
  74   41:     pub fn new(width: usize, height: usize, birth: Vec<usize>, survival: Vec<usize>) -> Self {
  75     :         let random_grid = Grid::from_vec(
  76     :             (0..width * height)
  77     :                 .map(|_| -> Cell {
  78     :                     match fastrand::bool() {
  79     :                         true => Cell::Alive(random_color()),
  80     :                         false => Cell::Dead,
  81     :                     }
  82     :                 })
  83     :                 .collect(),
  84     :             width,
       42:         fastrand::seed(
       43:             SystemTime::now()
       44:                 .duration_since(UNIX_EPOCH)
       45:                 .unwrap_or(Duration::new(1, 0))
       46:                 .as_millis()
       47:                 .try_into()
       48:                 .unwrap_or_default(),
  85   49:         );
       50: 
       51:         let random_grid = Array::from_shape_simple_fn((height, width), || match fastrand::bool() {
       52:             true => Cell::Alive(random_color()),
       53:             false => Cell::Dead,
       54:         });
  86   55:         info!(
  87   56:             "grid initialized\nsize:({}, {})\nbirth: {:?}\nsurvival: {:?}\ndensity: {}",
  88   57:             width,
    ...
  98   67:             grid: random_grid,
  99   68:             birth,
 100   69:             survival,
 101     :         }
 102     :     }
 103     : 
 104     :     pub fn step(&mut self) {
 105     :         let grid = &self.grid;
 106     :         let mut new_grid = grid.clone();
 107     : 
 108     :         new_grid.indexed_iter_mut().for_each(|((row, col), cell)| {
 109     :             let neighbors = self.get_neighbors(row, col);
 110     :             *cell = cell.with_neighbors(neighbors, &self.survival, &self.birth);
 111     :         });
       70:             last_tick: Instant::now(),
       71:             glider_last_tick: Instant::now(),
       72:         }
       73:     }
       74: 
       75:     fn padded_mirror_grid(&self, pad: usize) -> Array2<Cell> {
       76:         let (rows, cols) = self.grid.dim();
       77: 
       78:         Array2::from_shape_fn((rows + 2 * pad, cols + 2 * pad), |(i, j)| {
       79:             let mirrored_i = if i < pad {
       80:                 pad - i - 1
       81:             } else if i >= rows + pad {
       82:                 2 * rows + pad - i - 1
       83:             } else {
       84:                 i - pad
       85:             };
       86: 
       87:             let mirrored_j = if j < pad {
       88:                 pad - j - 1
       89:             } else if j >= cols + pad {
       90:                 2 * cols + pad - j - 1
       91:             } else {
       92:                 j - pad
       93:             };
       94: 
       95:             self.grid[(mirrored_i, mirrored_j)].clone()
       96:         })
       97:     }
       98: 
       99:     pub fn update(&mut self) {
      100:         if self.last_tick.elapsed() >= TIME_STEP {
      101:             self.last_tick = Instant::now();
      102:             self.step();
      103:         }
      104:         if self.glider_last_tick.elapsed() >= GLIDER_TIME_STEP {
      105:             self.glider_last_tick = Instant::now();
      106:             self.spawn_glider();
      107:         }
      108:     }
      109: 
      110:     fn step(&mut self) {
      111:         let mut new_grid = self.grid.clone();
      112:         Zip::from(&mut new_grid)
      113:             .and(self.padded_mirror_grid(1).windows((3, 3)))
      114:             .for_each(|new_cell, window| {
      115:                 let window_slice: Vec<&Cell> = window.iter().collect();
      116:                 *new_cell = with_neighbors(window_slice, &self.survival, &self.birth);
      117:             });
 112  118:         self.grid = new_grid
 113  119:     }
 114     : }
 115     : 
 116     : fn add_and_cast(a: usize, b: i8) -> Option<usize> {
 117     :     let b_as_isize = b as isize;
 118     :     a.checked_add_signed(b_as_isize)
      120: 
      121:     fn spawn_glider(&mut self) {
      122:         // empty area for a glider to spawn in
      123:         const GLIDER_AREA: usize = 5;
      124:         const GLIDER_PATTERNS: [[[bool; 3]; 3]; 4] = [
      125:             [
      126:                 [false, false, true],
      127:                 [true, false, true],
      128:                 [false, true, true],
      129:             ],
      130:             [
      131:                 [false, true, false],
      132:                 [true, false, false],
      133:                 [true, true, true],
      134:             ],
      135:             [
      136:                 [true, true, false],
      137:                 [true, false, true],
      138:                 [true, false, false],
      139:             ],
      140:             [
      141:                 [true, true, true],
      142:                 [false, false, true],
      143:                 [false, true, false],
      144:             ],
      145:         ];
      146: 
      147:         let (rows, cols) = self.grid.dim();
      148: 
      149:         // Get all window positions for GLIDER_AREA^2
      150:         let mut positions: Vec<(usize, usize)> = (0..=rows - GLIDER_AREA)
      151:             .flat_map(|i| (0..=cols - GLIDER_AREA).map(move |j| (i, j)))
      152:             .collect();
      153: 
      154:         fastrand::shuffle(&mut positions);
      155: 
      156:         for (start_row, start_col) in positions {
      157:             let is_all_dead = (0..GLIDER_AREA).all(|i| {
      158:                 (0..GLIDER_AREA)
      159:                     .all(|j| matches!(self.grid[(start_row + i, start_col + j)], Cell::Dead))
      160:             });
      161: 
      162:             if is_all_dead {
      163:                 // Choose a random pattern
      164:                 let glider_pattern = &GLIDER_PATTERNS[fastrand::usize(..4)];
      165: 
      166:                 let center_row = start_row + GLIDER_AREA / 2;
      167:                 let center_col = start_col + GLIDER_AREA / 2;
      168: 
      169:                 // Fill center with a random glider orientation
      170:                 (0..3).for_each(|i| {
      171:                     for j in 0..3 {
      172:                         if glider_pattern[i][j] {
      173:                             self.grid[(center_row + i - 1, center_col + j - 1)] =
      174:                                 Cell::Alive(random_color());
      175:                         }
      176:                     }
      177:                 });
      178: 
      179:                 info!(
      180:                     "Glider spawned at center: ({}, {}), with random rotation",
      181:                     center_row, center_col
      182:                 );
      183:                 return;
      184:             }
      185:         }
      186: 
      187:         info!("No suitable location found for spawning a glider.");
      188:     }
 119  189: }
Modified regular file src/cell.rs:
    ...
   3    3: #[derive(Debug, Clone)]
   4    4: pub enum Cell {
   5    5:     Alive(Color),
        6:     Growing((Color, f64)),
   6    7:     Dead,
   7    8: }
   8    9: 
   9   10: impl Cell {
  10     :     pub fn with_neighbors(
  11     :         &self,
  12     :         neighbors: Vec<&Cell>,
  13     :         survival: &[usize],
  14     :         birth: &[usize],
  15     :     ) -> Cell {
  16     :         let neighbor_count = neighbors
  17     :             .iter()
  18     :             .filter(|cell| match cell {
  19     :                 Cell::Alive(_) => true,
  20     :                 Cell::Dead => false,
  21     :             })
  22     :             .count();
  23     :         match self {
  24     :             Cell::Alive(_) => {
  25     :                 if survival.contains(&neighbor_count) {
  26     :                     self.clone()
  27     :                 } else {
  28     :                     Cell::Dead
  29     :                 }
       11:     /// Returns `true` if the cell is [`Alive`].
       12:     ///
       13:     /// [`Alive`]: Cell::Alive
       14:     #[must_use]
       15:     pub fn is_alive(&self) -> bool {
       16:         matches!(self, Self::Alive(..))
       17:     }
       18: 
       19:     /// Returns `true` if the cell is [`Dead`].
       20:     ///
       21:     /// [`Dead`]: Cell::Dead
       22:     #[must_use]
       23:     pub fn is_dead(&self) -> bool {
       24:         matches!(self, Self::Dead)
       25:     }
       26: }
       27: 
       28: pub fn with_neighbors(window: Vec<&Cell>, survival: &[usize], birth: &[usize]) -> Cell {
       29:     let center = window[4];
       30:     let neighbors = window
       31:         .iter()
       32:         .enumerate()
       33:         .filter_map(|(i, cell)| if i == 4 { None } else { Some(cell) });
       34:     match center {
       35:         Cell::Alive(_) => {
       36:             if survival.contains(&neighbors.filter(|cell| cell.is_alive()).count()) {
       37:                 center.clone()
       38:             } else {
       39:                 Cell::Dead
  30   40:             }
  31   41:         }
  31   42:         Cell::Growing((color, growth)) => Cell::Growing((*color, growth + 0.1)),
  31   43:         Cell::Dead => {
  32   44:                 if birth.contains(&neighbor_count) {
  33   44:                     let neighbor_colorsalive_neighbors: Vec<&Color> = neighbors
  34   45:                         .iter()
  35   45:                         .filter_map(|cell| match cell {
  36   46:                             Cell::Alive(color) => Some(color),
  37   47:                             Cell::Growing(_) => None,
  37   48:                     Cell::Dead => None,
  38   49:                         })
  39   50:                         .collect();
  40   51:             if birth.contains(&alive_neighbors.len()) {
  40   52:                 Cell::Alive(mix_colors(neighbor_colorsalive_neighbors))
  41   53:                 } else {
  42   54:                     selfcenter.clone()
  43     :                 }
  44   55:             }
  45   56:         }
  46   57:     }
    ...
Modified regular file src/main.rs:
    ...
  42   42: 
  43   43:     // this code runs on every draw ("frame")
  44   44:     terminal.draw_web(move |frame| {
  45   45:         automata.stepupdate();
  46   46: 
  47   47:         let area = if is_mobile() {
  48   48:             let vertical = Layout::vertical([Constraint::Percentage(30)]).flex(Flex::Center);
    ...
 130  130: 
 131  131: fn render_links(frame: &mut Frame<'_>, links_area: Rect) {
 132  132:     frame.render_widget(Block::bordered().title("Links".bold()), links_area);
 133  133:     for (i, (_name, url)) in LINKS.iter().enumerate() {
 134  134:         let link = Hyperlink::new((*name).underlined(), *url);
 135  135:         frame.render_widget(
 136  136:             link,
 137  137:             // offset to not overlay on the border
    ...
 145  145: 
 146  146: fn render_skills(frame: &mut Frame<'_>, meetups_area: Rect) {
 147  147:     frame.render_widget(
 148  148:         Paragraph::new("Making crazy shitstuff").block(Block::bordered().title("Skills".bold())),
 149  149:         meetups_area,
 150  150:     );
 151  151: }
    ...
