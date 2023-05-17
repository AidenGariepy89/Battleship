use crossterm::{
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::{self, Stdout};
use tui::{backend::CrosstermBackend, layout, style::{Color, Style}, widgets, Terminal};

use crate::{
    board::{BOARD_TOTAL, BOARD_WIDTH, self},
    game::GameData,
};

pub type Term = Terminal<CrosstermBackend<Stdout>>;

pub fn setup_terminal() -> Result<Term, io::Error> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    Ok(terminal)
}

pub fn destroy_terminal(mut terminal: Term) -> Result<(), io::Error> {
    disable_raw_mode()?;

    terminal
        .backend_mut()
        .execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

pub fn render_turn(term: &mut Term, player: crate::general::Player) -> io::Result<()> {
    term.draw(|f| {
        let cols = layout::Layout::default()
            .direction(layout::Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    layout::Constraint::Percentage(15),
                    layout::Constraint::Percentage(70),
                    layout::Constraint::Percentage(15),
                ]
                .as_ref(),
            )
            .split(f.size());
        let area = layout::Layout::default()
            .direction(layout::Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    layout::Constraint::Percentage(40),
                    layout::Constraint::Percentage(10),
                    layout::Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(cols[1])[1];
        let mut line = String::new();
        for _ in 0..area.width {
            line += "-";
        }
        let message = match player {
            crate::general::Player::One => line.clone() + "\nPlayer One\n" + &line,
            crate::general::Player::Two => line.clone() + "\nPlayer Two\n" + &line,
        };
        let message = widgets::Paragraph::new(message)
            .style(Style::default().fg(Color::Yellow))
            .alignment(layout::Alignment::Center);
        f.render_widget(message, area);
    })?;

    Ok(())
}

pub fn render_board(data: &mut GameData) -> io::Result<()> {
    let (screen1, screen2) = board::draw_board(data);

    data.term.draw(|f| {
        let height = f.size().height;
        let upper_board;
        let lower_board;
        if height > 22 {
            let extra_space = height - 22;
            let third_approx = extra_space / 3;
            upper_board = layout::Rect::new(0, third_approx, f.size().width, 11);
            lower_board = layout::Rect::new(0, height - third_approx - 11, f.size().width, 11);
        } else {
            upper_board = layout::Rect::new(0, 0, f.size().width, 11);
            lower_board = layout::Rect::new(0, height - 11, f.size().width, 11);
        }
        let base1 = widgets::Paragraph::new(screen1).alignment(layout::Alignment::Center);
        let base2 = widgets::Paragraph::new(screen2).alignment(layout::Alignment::Center);
        f.render_widget(base1, upper_board);
        f.render_widget(base2, lower_board);
    })?;

    Ok(())
}

pub fn render_clean_board(term: &mut Term, ships: &[bool; BOARD_TOTAL]) -> io::Result<()> {
    let mut out = String::new();
    out += " 1 2 3 4 5 6 7 8 9 0\n";
    for i in 0..BOARD_WIDTH {
        let mut line = (('A' as u8 + i as u8) as char).to_string() + " ";
        for j in 0..BOARD_WIDTH {
            if ships[(i * BOARD_WIDTH) + j] {
                line += "██";
            } else {
                line += ". ";
            }
        }
        line += "\n";
        out += &line;
    }

    term.draw(|f| {
        let spacing = (f.size().height - 11) / 2;
        let board = layout::Rect::new(0, spacing, f.size().width, 11);
        let p = widgets::Paragraph::new(out).alignment(layout::Alignment::Center);
        f.render_widget(p, board);
    })?;

    Ok(())
}
