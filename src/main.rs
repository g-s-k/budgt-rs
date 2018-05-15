extern crate termion;
extern crate tui;

use std::io;

use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Row, Table, Tabs, Widget};
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

    let datapts: Vec<_> = (1..100)
        .map(|x| x as f64)
        .map(|x| (x, (x * 3.14159 / 20.0).sin()))
        .collect();

    let tbl_data = vec![
        TransactionInstance::new("foo", 123.45, "bar", 100.0, "baz", 355.02),
        TransactionInstance::new("blat", 23.99, "scram", 0.56, "", 0.0),
        TransactionInstance::new("fizz", 15.0, "", 0.0, "buzz", 16.98),
    ];

    let tbl_fmt = tbl_data.iter().map(|ref row| {
        Row::Data(row.fmt_table().into_iter())
    });

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
                .datasets(&[
                    Dataset::default()
                        .data(datapts.as_ref())
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Yellow)),
                ])
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
                        .widths(&[10, 10, 10, 10, 10, 10])
                        .column_spacing(1)
                        .render(t, &chunks2[0]);
                });
        });

    t.draw()
}

struct AccountSnapshot(String, f64);

struct TransactionInstance {
    name: String,
    date: String,
    amount: f64,
    source: Option<AccountSnapshot>,
    dest: Option<AccountSnapshot>,
}

impl TransactionInstance {
    fn new(
        name: &str,
        amount: f64,
        source: &str,
        s_balance: f64,
        dest: &str,
        d_balance: f64,
    ) -> TransactionInstance {
        let source = match source {
            "" => None,
            name => Some(AccountSnapshot(name.to_string(), s_balance)),
        };
        let dest = match dest {
            "" => None,
            name => Some(AccountSnapshot(name.to_string(), d_balance)),
        };

        TransactionInstance {
            name: name.to_string(),
            date: "".to_string(),
            amount,
            source,
            dest,
        }
    }

    fn fmt_table(&self) -> Vec<String> {
    vec![
        self.name.clone(),
        format!("{:8}", self.amount),

        match self.source {
            Some(ref acct) => acct.0.clone(),
            None => "".to_string()
        },

        if let Some(ref acct) = self.source {
            format!("{:8}", acct.1)
        } else {
            "".to_string()
        },

        match self.dest {
            Some(ref acct) => acct.0.clone(),
            None => "".to_string()
        },

        if let Some(ref acct) = self.dest {
            format!("{:8}", acct.1)
        } else {
            "".to_string()
        }
    ]

    }
}
