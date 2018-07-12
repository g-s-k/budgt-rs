extern crate budgt;
extern crate termion;
extern crate tui;

use std::io;

use std::sync::mpsc;
use std::thread;

use termion::event;
use termion::input::TermRead;

use tui::backend::RawBackend;
use tui::layout::{Direction, Group, Size};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Row, Table, Tabs, Widget};
use tui::Terminal;

use budgt::{AccountSnapshot, Money, TransactionInstance};

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

    let datapts: Vec<_> = (1..100)
        .map(|x| x as f64)
        .map(|x| (x, (x * 3.14159 / 20.0).sin()))
        .collect();

    let tbl_data: Vec<TransactionInstance> = vec![
        TransactionInstance::default()
            .name("foo")
            .amount(12345)
            .source(AccountSnapshot("bar".to_string(), Money(1000)))
            .dest(AccountSnapshot("baz".to_string(), Money(35502))),
        TransactionInstance::default()
            .name("blat")
            .amount(2399)
            .source(AccountSnapshot("scram".to_string(), Money(56))),
        TransactionInstance::default()
            .name("fizz")
            .amount(1500)
            .dest(AccountSnapshot("buzz".to_string(), Money(1698))),
        TransactionInstance::default()
            .name("FreeBSD")
            .amount(1200),
    ];

    let tbl_fmt = tbl_data
        .iter()
        .map(|ref row| Row::Data(row.fmt_table().into_iter()));

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(33), Size::Fixed(3), Size::Min(13)])
        .render(t, &size, |t, chunks| {
            Chart::default()
                .block(
                    Block::default()
                        .title("Projected Balances")
                        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::Bold)),
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
                .datasets(&[Dataset::default()
                    .data(datapts.as_ref())
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))])
                .render(t, &chunks[0]);
            Tabs::default()
                .block(Block::default().borders(Borders::ALL))
                .titles(&["Transactions", "Ending Balances"])
                .highlight_style(Style::default().fg(Color::Cyan))
                .select(0)
                .render(t, &chunks[1]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Percent(100)])
                .render(t, &chunks[2], |t, chunks2| {
                    Table::new(
                        [
                            "Date",
                            "Name",
                            "Amount",
                            "Source",
                            "(Balance)",
                            "Dest.",
                            "(Balance)",
                        ].into_iter(),
                        tbl_fmt,
                    ).block(Block::default())
                        .header_style(Style::default().modifier(Modifier::Bold))
                        .widths(&[10, 10, 10, 10, 10, 10, 10])
                        .column_spacing(1)
                        .render(t, &chunks2[0]);
                });
        });

    t.draw()
}
