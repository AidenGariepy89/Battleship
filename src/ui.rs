use crossterm::{terminal::{self, enable_raw_mode, disable_raw_mode}, ExecutableCommand, event};
use std::io::{self, Stdout};
use tui::{backend::CrosstermBackend, layout, style, widgets, Terminal};

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
            crate::general::Player::One => {
                line.clone() + "\nPlayer One\n" + &line
            }
            crate::general::Player::Two => {
                line.clone() + "\nPlayer Two\n" + &line
            }
        };
        let message = widgets::Paragraph::new(message)
            .style(style::Style::default().fg(style::Color::Yellow))
            .alignment(layout::Alignment::Center);
        f.render_widget(message, area);
    })?;

    event::read()?;

    Ok(())
}

