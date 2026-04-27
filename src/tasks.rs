use crate::state::SharedState;
use crate::engine::BioEngine;
use std::fs::File;
use std::io::Write;

pub async fn analyze_sequences(state: SharedState) {
    let data = {
        let s = state.read().await;
        s.sequences.iter().enumerate().map(|(i, r)| (i, r.raw.clone())).collect::<Vec<_>>()
    };

    let total = data.len();
    for (i, (idx, raw)) in data.into_iter().enumerate() {
        {
            let mut s = state.write().await;
            if let Some(rec) = s.sequences.get(idx) {
                let id = rec.id.clone();
                s.add_log(format!("Analyzing {}...", id));
            }
            s.analysis_progress = i as f64 / total as f64;
        }

        let profile = BioEngine::profile_sequence(&raw);
        
        let mut s = state.write().await;
        if let Some(rec) = s.sequences.get_mut(idx) { rec.profile = Some(profile); }
        s.analysis_progress = (i + 1) as f64 / total as f64;
    }
    state.write().await.add_log("✓ Analysis complete");
}

pub async fn clear_profiles(state: SharedState) {
    let mut s = state.write().await;
    for r in s.sequences.iter_mut() { r.profile = None; }
    s.analysis_progress = 0.0;
    s.add_log("Profiles cleared");
}

pub async fn export_tsv(state: SharedState) {
    let s = state.read().await;
    if let Ok(mut f) = File::create("helix_export.tsv") {
        let _ = writeln!(f, "ID\tLength\tGC_Pct\tORFs_Found");
        for r in &s.sequences {
            if let Some(p) = &r.profile {
                let _ = writeln!(f, "{}\t{}\t{:.4}\t{}", r.id, r.length, p.gc_content, p.orfs.len());
            }
        }
        drop(s);
        state.write().await.add_log("✓ Exported to helix_export.tsv");
    }
}