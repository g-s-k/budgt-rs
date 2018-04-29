extern crate termion;
extern crate tui;

use std::io;

use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::layout::{Direction, Group, Size};
use tui::style::{Color, Modifier, Style};

fn main() {
    let mut terminal = init().expect("Failed initialization.");
    chk_term_size(&terminal);

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

fn chk_term_size(t: &Terminal<RawBackend>) {
    let size = t.size().unwrap();
    if size.width < 80 || size.height < 24 {
        panic!("Terminal must be at least 80 x 24.");
    }
}

fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
    let size = t.size()?;

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(33), Size::Percent(67)])
        .render(t, &size, |t, chunks| {
            let datapts: Vec<_> = (1..100)
                .map(|x| x as f64)
                .map(|x| (x, (x * 3.14159 / 20.0).sin()))
                .collect();

            Chart::default()
                .block(
                    Block::default()
                        .title("Graph")
                        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::Bold))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .bounds([0.0, 100.0])
                        .labels(&["0", "25", "50", "75", "100"]),
                )
                .y_axis(
                    Axis::default()
                        .bounds([-2.0, 2.0])
                        .labels(&["-2", "0", "2"]),
                )
                .datasets(&[
                    Dataset::default()
                        .data(datapts.as_ref())
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Yellow)),
                ])
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
