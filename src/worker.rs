use tokio::sync::mpsc;
use crate::fasta;
use crate::state::SharedState;
use crate::tasks;

pub enum Job {
    Analyze,
    LoadFasta(String),
    LoadDemo,
    ClearProfiles,
    ClearAll,
}

pub async fn start_worker(mut rx: mpsc::Receiver<Job>, state: SharedState) {
    while let Some(job) = rx.recv().await {
        match job {
            Job::Analyze => {
                tasks::analyze_sequences(state.clone()).await;
            }
            Job::LoadFasta(path) => {
                match fasta::load_file(&path) {
                    Ok(new_seqs) => {
                        let mut s = state.write().await;
                        s.sequences.extend(new_seqs);
                        s.add_log(format!("✓ Appended sequences from {}", path));
                    }
                    Err(e) => {
                        let mut s = state.write().await;
                        s.error = Some(format!("Load failed: {}", e));
                    }
                }
            }
            Job::LoadDemo => {
                let demo = fasta::demo_sequences();
                let mut s = state.write().await;
                let count = demo.len();
                s.sequences.extend(demo);
                s.add_log(format!("✓ Background worker loaded {} demo sequences", count));
            }
            Job::ClearProfiles => {
                tasks::clear_profiles(state.clone()).await;
            }
            Job::ClearAll => {
                let mut s = state.write().await;
                s.sequences.clear();
                s.selected = 0;
                s.add_log("🗑 Workspace cleared");
            }
        }
    }
}