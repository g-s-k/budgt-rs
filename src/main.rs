extern crate tui;

use std::io;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Group, Size, Direction};

fn main() {
    let mut terminal = init().expect("Failed initialization.");
    
    terminal.clear().expect("Failed to clear the terminal window.");

    draw(&mut terminal).expect("Failed to draw");
}

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new().unwrap();
    Terminal::new(backend)
}

fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
    let size = t.size()?;

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(33), Size::Percent(67)])
        .render(t, &size, |t, chunks| {
            Block::default()
                .title("Graph")
                .borders(Borders::ALL)
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Percent(62), Size::Percent(38)])
                .render(t, &chunks[1], |t, chunks2| {
                    Block::default()
                        .title("Transaction Record")
                        .borders(Borders::ALL)
                        .render(t, &chunks2[0]);
                    Block::default()
                        .title("Balances")
                        .borders(Borders::ALL)
                        .render(t, &chunks2[1]);
                });
        });

    t.draw()
}
