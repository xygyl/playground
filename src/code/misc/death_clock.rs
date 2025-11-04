use crossterm::event::{self, Event};
use ratatui::{
    init, restore,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use std::{io::Result, str::FromStr, time::Duration};

const MAX: u8 = 100;

pub fn run_death_clock() -> Result<()> {
    let terminal = init();
    let result = run(terminal);
    restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut counter = [0u8; 4];
    loop {
        let nums = death_clock(&mut counter);
        terminal.draw(|frame| render(frame, nums))?;

        if event::poll(Duration::from_millis(10))? {
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame, nums: [u8; 4]) {
    frame.render_widget(
        death_clock_display(nums).block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(
                    Style::default().fg(Color::from_str("#312A50").unwrap_or(Color::Reset)),
                )
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(1)),
        ),
        frame.area(),
    );
}

fn death_clock(counter: &mut [u8; 4]) -> [u8; 4] {
    counter[0] += 1;

    if counter[0] > MAX {
        counter[0] = 0;
        counter[1] += 1;
    }
    if counter[1] > MAX {
        counter[1] = 0;
        counter[2] += 1;
    }
    if counter[2] > MAX {
        counter[2] = 0;
        counter[3] += 1;
    }
    if counter[3] > MAX {
        *counter = [0, 0, 0, 0];
    }

    *counter
}

fn death_clock_display(nums: [u8; 4]) -> Paragraph<'static> {
    Paragraph::new(
        nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .style(Style::new().fg(Color::from_u32(0x00DDE1FF)))
}
