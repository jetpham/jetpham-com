mod automata;
mod cell;
use std::{cell::RefCell, io, rc::Rc};

use automata::Automata;
use ratzilla::ratatui::{
    Terminal,
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
};

use log::info;
use ratzilla::{DomBackend, WebRenderer, event::KeyCode};

fn main() -> io::Result<()> {
    wasm_log::init(wasm_log::Config::default());
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size().unwrap();
    //this uses the rules from conway's game of life
    let mut automata = Automata::new(size.width.into(), size.height.into(), vec![3], vec![2, 3]);
    info!("initiating meow");

    // terminal.on_key_event({
    //     move |key_event| {
    //         if key_event.code == KeyCode::Char(' ') {}
    //     }
    // });

    terminal.draw_web(move |f| {
        automata.step();
        f.render_widget(automata.life_canvas(f.area()), f.area());
    });

    Ok(())
}
