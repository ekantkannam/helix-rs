use std::{io, sync::Arc, time::Duration};
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::{mpsc, RwLock};

use crate::{
    state::{AppState, InputMode, SharedState},
    ui,
    worker::{self, Job},
};

pub async fn run_app(fasta_path: Option<String>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let result = run_inner(&mut terminal, fasta_path).await;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

async fn run_inner(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, fasta_path: Option<String>) -> anyhow::Result<()> {
    let state: SharedState = Arc::new(RwLock::new(AppState::default()));
    let (job_tx, job_rx) = mpsc::channel::<Job>(64);
    { let s = state.clone(); tokio::spawn(worker::start_worker(job_rx, s)); }

    let (ev_tx, mut ev_rx) = mpsc::channel::<Event>(128);
    std::thread::spawn(move || {
        loop {
            if let Ok(true) = crossterm::event::poll(Duration::from_millis(50)) {
                if let Ok(ev) = crossterm::event::read() { if ev_tx.blocking_send(ev).is_err() { break; } }
            }
        }
    });

    if let Some(path) = fasta_path { let _ = job_tx.send(Job::LoadFasta(path)).await; }
    let mut tick = tokio::time::interval(Duration::from_millis(80));

    loop {
        { let s = state.read().await; terminal.draw(|f| ui::draw(f, &s))?; }
        tokio::select! {
            _ = tick.tick() => { 
                let mut s = state.write().await;
                s.spinner_tick = s.spinner_tick.wrapping_add(1); 
            }
            Some(event) = ev_rx.recv() => {
                if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event {
                    if kind != KeyEventKind::Press { continue; }
                    if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') { break; }

                    let mode = state.read().await.input_mode.clone();
                    match mode {
                        InputMode::Normal => match code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            
                            // THE FIX: Listen for 'l', 'L', OR '1'
                            KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Char('1') => { 
                                let mut s = state.write().await; 
                                s.input_mode = InputMode::Editing; 
                                s.input.clear(); 
                                s.add_log("Opening file loader...");
                            }

                            KeyCode::Char('/') => { let mut s = state.write().await; s.input_mode = InputMode::Searching; s.search_query.clear(); }
                            KeyCode::Char('r') => { let _ = job_tx.send(Job::Analyze).await; }
                            KeyCode::Char('d') => { let _ = job_tx.send(Job::LoadDemo).await; }
                            KeyCode::Char('e') => { let s = state.clone(); tokio::spawn(async move { crate::tasks::export_tsv(s).await; }); }
                            KeyCode::Char('c') => { let _ = job_tx.send(Job::ClearProfiles).await; }
                            KeyCode::Char('C') => { let _ = job_tx.send(Job::ClearAll).await; }
                            
                            // Toggle views (exclusive)
                            KeyCode::Char('v') => { 
                                let mut s = state.write().await; 
                                s.show_revcomp = !s.show_revcomp; 
                                s.show_translation = false; 
                            }
                            KeyCode::Char('t') => { 
                                let mut s = state.write().await; 
                                s.show_translation = !s.show_translation; 
                                s.show_revcomp = false; 
                            }

                            KeyCode::Down | KeyCode::Char('j') => {
                                let mut s = state.write().await;
                                let q = s.search_query.to_lowercase();
                                if let Some((i, _)) = s.sequences.iter().enumerate().skip(s.selected + 1).find(|(_, r)| q.is_empty() || r.id.to_lowercase().contains(&q)) { s.selected = i; }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                let mut s = state.write().await;
                                let q = s.search_query.to_lowercase();
                                if let Some((i, _)) = s.sequences.iter().enumerate().take(s.selected).rev().find(|(_, r)| q.is_empty() || r.id.to_lowercase().contains(&q)) { s.selected = i; }
                            }
                            _ => {}
                        },
                        InputMode::Editing => match code {
                            KeyCode::Enter => {
                                let path = { let mut s = state.write().await; s.input_mode = InputMode::Normal; s.input.clone() };
                                if !path.trim().is_empty() { let _ = job_tx.send(Job::LoadFasta(path)).await; }
                            }
                            KeyCode::Char(c) => { state.write().await.input.push(c); }
                            KeyCode::Backspace => { state.write().await.input.pop(); }
                            KeyCode::Esc => { state.write().await.input_mode = InputMode::Normal; }
                            _ => {}
                        },
                        InputMode::Searching => match code {
                            KeyCode::Enter | KeyCode::Esc => { state.write().await.input_mode = InputMode::Normal; }
                            KeyCode::Char(c) => { state.write().await.search_query.push(c); }
                            KeyCode::Backspace => { state.write().await.search_query.pop(); }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}