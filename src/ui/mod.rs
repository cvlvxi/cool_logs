use std::{io::stdout, time::Duration};
use eyre::Result;
use tokio::sync::mpsc;
use tui::{backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::{Layout, Direction, Constraint, Alignment}, widgets::{Paragraph, Block, Borders, BorderType}, style::{Style, Color}, text::{Spans, Span}};
use tui_logger::TuiLoggerWidget;
use log::{debug, error, warn};

use crate::logging::LineParts;



// pub enum AppReturn {
//     Exit,
//     Continue,
// }

pub async fn start_ui(log_rx: &mut mpsc::Receiver<LineParts>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // User event handler
    let tick_rate = Duration::from_millis(200);

    loop {
        // Render
        terminal.draw(|rect| draw(rect))?;

        // Handle inputs
        // let result = match events.next().await {
        //     InputEvent::Input(key) => app.do_action(key).await,
        //     InputEvent::Tick => app.update_on_tick().await,
        // };
        // Check if we should exit
        // if result == AppReturn::Exit {
        //     events.close();
        //     break;
        // }
        if let Some(i) = log_rx.recv().await {
            debug!("Got the line parts: {:?}", i);
            debug!("{:?}", i);
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    
    Ok(())
    
}

pub fn draw<B>(rect: &mut Frame<B>)
where
    B: Backend,
{
    let size = rect.size();

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body & Help
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let body = draw_body();
    rect.render_widget(body, body_chunks[0]);

    // Logs
    let logs = draw_logs();
    rect.render_widget(logs, chunks[3]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Cool Logs")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_body<'a>() -> Paragraph<'a> {
    let initialized_text = "Initialized";
    let loading_text = "Loading...";
    let sleep_text = format!("Sleep count: {}", 1);

    let tick_text = format!("Tick count: {}", 100);
    
    Paragraph::new(vec![
        Spans::from(Span::raw(initialized_text)),
        Spans::from(Span::raw(loading_text)),
        Spans::from(Span::raw(sleep_text)),
        Spans::from(Span::raw(tick_text)),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            // .title("Body")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(tui::style::Style::default().fg(Color::Red))
        .style_debug(tui::style::Style::default().fg(Color::Green))
        .style_warn(tui::style::Style::default().fg(Color::Yellow))
        .style_trace(tui::style::Style::default().fg(Color::Gray))
        .style_info(tui::style::Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(tui::style::Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(tui::style::Style::default().fg(Color::White).bg(Color::Black))
}
