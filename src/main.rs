extern crate termion;
extern crate tui;

use std::io;

use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Block, Borders, Widget};
use tui::layout::{Direction, Group, Size};

fn main() {
    let mut terminal = init().expect("Failed initialization.");

    // listener for keys
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || {
        let stdin = io::stdin();

        for ch in stdin.keys() {
            match ch.unwrap() {
                event::Key::Char('q') => {
                    input_tx.send("quit").unwrap();
                }
                _ => {}
            }
        }
    });

    // get ready to run
    terminal
        .clear()
        .expect("Failed to clear the terminal window.");

    terminal.hide_cursor().expect("Failed to hide the cursor.");

    // actually run the thing
    loop {
        draw(&mut terminal).expect("Failed to draw.");

        let evt = rx.recv().unwrap();
        match evt {
            "quit" => break,
            _ => {}
        }
    }

    // clean up at the end
    terminal.show_cursor().expect("Failed to show the cursor.");

    terminal
        .clear()
        .expect("Failed to clear the terminal window.");
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
