use crate::state::{Orf, SequenceKind, SequenceProfile};

pub struct BioEngine;

impl BioEngine {
    pub fn profile_sequence(seq: &[u8]) -> SequenceProfile {
        let mut gc = 0;
        for &b in seq {
            if matches!(b.to_ascii_uppercase(), b'G' | b'C') { gc += 1; }
        }
        let gc_content = if !seq.is_empty() { gc as f64 / seq.len() as f64 } else { 0.0 };
        
        SequenceProfile {
            gc_content,
            kind: SequenceKind::Dna,
            orfs: Self::find_orfs(seq, 90),
            gc_window_data: Self::calculate_sliding_window_gc(seq, 1000, 500),
        }
    }

    fn calculate_sliding_window_gc(seq: &[u8], window: usize, step: usize) -> Vec<f64> {
        if seq.len() < window { return vec![0.5]; }
        let mut data = Vec::new();
        for i in (0..seq.len() - window).step_by(step) {
            let gc = seq[i..i+window].iter().filter(|&&b| matches!(b.to_ascii_uppercase(), b'G' | b'C')).count();
            data.push(gc as f64 / window as f64);
        }
        data
    }

    pub fn translate_dna(seq: &[u8]) -> String {
        seq.chunks_exact(3).map(|c| {
            match &c.to_ascii_uppercase()[..] {
                b"ATG" => 'M', b"TAA" | b"TAG" | b"TGA" => '*',
                b"GCA"|b"GCC"|b"GCG"|b"GCT" => 'A', b"TTC"|b"TTT" => 'F',
                b"GAA"|b"GAG" => 'E', b"GAT"|b"GAC" => 'D',
                _ => '.',
            }
        }).collect()
    }

    pub fn find_orfs(seq: &[u8], min: usize) -> Vec<Orf> {
        let mut orfs = Vec::new();
        for frame in 0..3 {
            let mut i = frame;
            let mut start = None;
            while i + 2 < seq.len() {
                let codon = &seq[i..i+3].to_ascii_uppercase();
                if start.is_none() && codon == b"ATG" { start = Some(i); }
                else if let Some(s) = start {
                    if codon == b"TAA" || codon == b"TAG" || codon == b"TGA" {
                        if (i+3-s) >= min { orfs.push(Orf { start: s, end: i+3, length: i+3-s, frame: (frame as u8)+1 }); }
                        start = None;
                    }
                }
                i += 3;
            }
        }
        orfs
    }
}