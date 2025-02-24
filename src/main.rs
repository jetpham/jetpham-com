mod automata;
mod cell;
use std::io;

use automata::Automata;
use ratzilla::ratatui::Terminal;

use log::info;
use ratzilla::{DomBackend, WebRenderer};

fn main() -> io::Result<()> {
    wasm_log::init(wasm_log::Config::default());
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size().unwrap();
    //this uses the rules from conway's game of life
    let mut automata = Automata::new(size.width.into(), size.height.into(), vec![3], vec![2, 3]);
    info!("running automata");
    terminal.draw_web(move |f| {
        automata.step();
        f.render_widget(automata.life_canvas(f.area()), f.area());
    });

    Ok(())
}
