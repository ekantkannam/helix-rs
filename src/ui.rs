use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear, Gauge},
    Frame,
};
use crate::state::{AppState, InputMode};

pub fn draw(f: &mut Frame, state: &AppState) {
    let area = f.size();
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(8), Constraint::Length(1)])
        .split(area);

    let gauge = Gauge::default()
        .block(Block::default().title(" Analysis Progress ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent((state.analysis_progress * 100.0) as u16);
    f.render_widget(gauge, root[0]);

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(root[1]);

    let items: Vec<ListItem> = state.sequences.iter().enumerate().map(|(i, r)| {
        let sym = if r.profile.is_some() { "●" } else { "○" };
        let style = if i == state.selected { Style::default().bg(Color::Rgb(40, 60, 80)) } else { Style::default() };
        ListItem::new(format!(" {} {}", sym, r.id)).style(style)
    }).collect();
    f.render_widget(List::new(items).block(Block::default().title(" Sequences ").borders(Borders::ALL)), main[0]);

    if let Some(rec) = state.selected_record() {
        let content = if let Some(p) = &rec.profile {
            format!("ID: {}\nLength: {} bp\nGC Content: {:.2}%\nORFs Detected: {}", rec.id, rec.length, p.gc_content * 100.0, p.orfs.len())
        } else {
            format!("ID: {}\nStatus: Not Analyzed (Press 'r')", rec.id)
        };
        f.render_widget(Paragraph::new(content).block(Block::default().title(" Analysis ").borders(Borders::ALL)), main[1]);
    }

    let logs: Vec<ListItem> = state.logs.iter().rev().take(6).map(|l| ListItem::new(l.as_str())).collect();
    f.render_widget(List::new(logs).block(Block::default().title(" Logs ").borders(Borders::ALL)), root[2]);

    if state.input_mode == InputMode::Editing {
        let modal = centered_rect(60, 20, area);
        f.render_widget(Clear, modal);
        f.render_widget(Paragraph::new(format!("Path: {}", state.input)).block(Block::default().borders(Borders::ALL).title(" Load File ")), modal);
    }
}

fn centered_rect(px: u16, py: u16, r: Rect) -> Rect {
    let v = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage((100 - py) / 2), Constraint::Percentage(py), Constraint::Percentage((100 - py) / 2)]).split(r);
    let h = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage((100 - px) / 2), Constraint::Percentage(px), Constraint::Percentage((100 - px) / 2)]).split(v[1]);
    h[1]
}