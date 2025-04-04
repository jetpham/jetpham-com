mod automata;
mod cell;
use std::io;

use automata::Automata;
use log::{info, warn};
use ratzilla::ratatui::Frame;
use ratzilla::ratatui::layout::{Alignment, Constraint, Flex, Layout, Offset, Rect};
use ratzilla::ratatui::style::{Color, Style, Stylize};
use ratzilla::ratatui::text::Line;
use ratzilla::ratatui::widgets::{BorderType, Clear, Paragraph, Wrap};
use ratzilla::ratatui::{Terminal, widgets::Block};
use ratzilla::utils::is_mobile;
use ratzilla::widgets::Hyperlink;
use ratzilla::{DomBackend, WebRenderer};
use std::iter::zip;

// I used https://patorjk.com/software/taag/#p=display&f=ANSI%20Shadow&t=Jet%20Pham%20 to generate this block
const NAME: &str = r#"
     ██╗███████╗████████╗    ██████╗ ██╗  ██╗ █████╗ ███╗   ███╗
     ██║██╔════╝╚══██╔══╝    ██╔══██╗██║  ██║██╔══██╗████╗ ████║
     ██║█████╗     ██║       ██████╔╝███████║███████║██╔████╔██║
██   ██║██╔══╝     ██║       ██╔═══╝ ██╔══██║██╔══██║██║╚██╔╝██║
╚█████╔╝███████╗   ██║       ██║     ██║  ██║██║  ██║██║ ╚═╝ ██║
 ╚════╝ ╚══════╝   ╚═╝       ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝
"#;

const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/jetpham"),
    ("LinkedIn", "https://www.linkedin.com/in/jetpham/"),
    ("Bluesky", "https://bsky.app/profile/jetpham.com"),
];

fn main() -> io::Result<()> {
    wasm_log::init(wasm_log::Config::default());
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    // this uses the rules from conway's game of life
    let mut automata = Automata::new(size.width.into(), size.height.into(), vec![3], vec![2, 3]);
    info!("running automata");

    // this code runs on every draw ("frame")
    terminal.draw_web(move |frame| {
        automata.update();

        let area = if is_mobile() {
            let vertical = Layout::vertical([Constraint::Percentage(30)]).flex(Flex::Center);
            let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
            let [area] = vertical.areas(frame.area());
            let [area] = horizontal.areas(area);
            area
        } else {
            let vertical = Layout::vertical([Constraint::Percentage(80)]).flex(Flex::Center);
            let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
            let [area] = vertical.areas(frame.area());
            let [area] = horizontal.areas(area);
            area
        };

        if is_mobile() {
            let constraints = [
                Constraint::Length(3),
                Constraint::Length(LINKS.len() as u16 + 2),
            ];
            render_background(frame, area, Some("┤Jet Pham├".to_string()), &constraints);
            let [meetups_area, links_area] = Layout::vertical(constraints).areas(area);
            render_skills(frame, meetups_area);
            render_links(frame, links_area);
        } else {
            let description = textwrap::wrap(DESCRIPTION.trim(), area.width as usize - 15)
                .iter()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            let constraints = [
                Constraint::Length(NAME.lines().count() as u16 + 1),
                Constraint::Length(description.lines().count() as u16 + 2),
                Constraint::Length(3),
                Constraint::Length(LINKS.len() as u16 + 2),
            ];
            render_background(frame, area, None, &constraints);
            let [banner_area, description_area, meetups_area, links_area] =
                Layout::vertical(constraints).areas(area);
            render_banner(frame, banner_area);
            render_description(frame, description, description_area);
            render_skills(frame, meetups_area);
            render_links(frame, links_area);
        }

        color_buffer(frame, automata.rat_colors());
    });
    Ok(())
}

fn contains_alpha_numeric_or_symbols(input: &str) -> bool {
    r#"
 ╭─╮│╗║╔═╝╚┌┐└┘╰┤├╯"#
        .contains(input)
}

fn color_buffer(frame: &mut Frame<'_>, colors: Vec<Option<Color>>) {
    let buffer_content = &mut frame.buffer_mut().content;
    if buffer_content.len() != colors.len() {
        warn!("color and buffer not the same ")
    }

    zip(buffer_content.iter_mut(), colors.iter()).for_each(|(buffer, color)| {
        if let Some(color) = color {
            if contains_alpha_numeric_or_symbols(buffer.symbol()) {
                buffer.bg = *color;
            } else if buffer.symbol() == "█" {
                buffer.bg = *color;
                buffer.set_symbol(" ");
            } else {
                buffer.fg = *color;
            }
        } else if buffer.symbol() == "█" {
            buffer.bg = Color::White;
            buffer.set_symbol(" ");
        }
    });
}

const DESCRIPTION: &str = r#"
Trains, planes, and automobiles are my jam

University of San Francisco - Computer Science
"#;

fn render_links(frame: &mut Frame<'_>, links_area: Rect) {
    frame.render_widget(Block::bordered().title("Links".bold()), links_area);
    for (i, (_name, url)) in LINKS.iter().enumerate() {
        let link = Hyperlink::new(*url);
        frame.render_widget(
            link,
            // offset to not overlay on the border
            links_area.offset(Offset {
                x: 1,
                y: i as i32 + 1,
            }),
        );
    }
}

fn render_skills(frame: &mut Frame<'_>, meetups_area: Rect) {
    frame.render_widget(
        Paragraph::new("Making crazy stuff").block(Block::bordered().title("Skills".bold())),
        meetups_area,
    );
}

fn render_description(frame: &mut Frame<'_>, description: String, description_area: Rect) {
    frame.render_widget(
        Paragraph::new(description)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered()),
        description_area,
    );
}

fn render_banner(frame: &mut Frame<'_>, banner_area: Rect) {
    frame.render_widget(
        Paragraph::new(NAME).alignment(Alignment::Center),
        banner_area,
    );
}

fn render_background(
    frame: &mut Frame<'_>,
    area: Rect,
    title: Option<String>,
    constraints: &[Constraint],
) {
    let mut area = Rect::new(
        area.x - 2,
        area.y - 1,
        area.width + 4,
        constraints
            .iter()
            .map(|c| match *c {
                Constraint::Min(v) | Constraint::Max(v) | Constraint::Length(v) => v,
                _ => 0,
            })
            .sum::<u16>()
            + 3,
    );
    area = area.clamp(frame.area());
    let mut block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Color::White)
        .style(Style::default().fg(Color::White).bg(Color::Reset))
        .title_bottom("┤meow├")
        .title_alignment(Alignment::Right);
    if let Some(title) = title {
        block = block.title_top(Line::from(title).alignment(Alignment::Center).bold());
    }
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}
