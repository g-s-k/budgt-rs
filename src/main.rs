extern crate tui;

use std::io;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Group, Size, Direction};

fn main() {
    let mut terminal = init().expect("Failed initialization.");
    draw(&mut terminal).expect("Failed to draw");
}

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new().unwrap();
    Terminal::new(backend)
}

fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
    let size = t.size()?;

    t.clear()?;

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(15), Size::Percent(70), Size::Percent(10)])
        .render(t, &size, |t, chunks| {
            Block::default()
                .title("Block 0")
                .borders(Borders::ALL)
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[1], |t, chunks2| {
                    Block::default()
                        .title("Block 1.0")
                        .borders(Borders::ALL)
                        .render(t, &chunks2[0]);
                    Block::default()
                        .title("Block 1.1")
                        .borders(Borders::ALL)
                        .render(t, &chunks2[1]);
                });
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .render(t, &chunks[2]);
        });

    t.draw()
}
