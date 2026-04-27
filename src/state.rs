use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum InputMode {
    #[default] Normal, Editing, Searching,
}

#[derive(Debug, Default, Clone)]
pub struct Orf {
    pub start: usize, pub end: usize, pub length: usize, pub frame: u8,
}

#[derive(Debug, Default, Clone)]
pub struct SequenceProfile {
    pub gc_content: f64,
    pub kind: SequenceKind,
    pub orfs: Vec<Orf>,
    pub gc_window_data: Vec<f64>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum SequenceKind {
    #[default] Unknown, Dna, Rna, Protein,
}

impl std::fmt::Display for SequenceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dna => write!(f, "DNA"),
            Self::Rna => write!(f, "RNA"),
            Self::Protein => write!(f, "Protein"),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SequenceRecord {
    pub id: String, 
    pub desc: String, 
    pub length: usize, 
    pub raw: Vec<u8>,
    pub profile: Option<SequenceProfile>,
}

impl SequenceRecord {
    pub fn new(id: String, desc: String, raw: Vec<u8>) -> Self {
        let length = raw.len();
        Self { id, desc, length, raw, profile: None }
    }
}

#[derive(Default)]
pub struct AppState {
    pub sequences: Vec<SequenceRecord>,
    pub selected: usize,
    pub logs: Vec<String>,
    pub loading: bool,
    pub spinner_tick: u8,
    pub error: Option<String>,
    pub input_mode: InputMode,
    pub input: String,
    pub search_query: String,
    pub show_revcomp: bool,
    pub show_translation: bool,
    pub analysis_progress: f64,
}

impl AppState {
    pub fn add_log(&mut self, msg: impl Into<String>) {
        self.logs.push(format!("[{}] {}", timestamp(), msg.into()));
        if self.logs.len() > 100 { self.logs.remove(0); }
    }
    pub fn selected_record(&self) -> Option<&SequenceRecord> {
        self.sequences.get(self.selected)
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

pub fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let hrs = (s / 3600) % 24;
    let mins = (s / 60) % 60;
    let secs = s % 60;
    format!("{:02}:{:02}:{:02}", hrs, mins, secs)
}