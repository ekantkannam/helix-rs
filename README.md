# Helix-rs: High-Performance Genomic Analysis TUI

![Rust](https://img.shields.io/badge/rust-v1.70%2B-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey?style=flat-square)

Helix-rs is a high-performance, locally executed Terminal User Interface (TUI) built in Rust for bioinformatics and genomic analysis. It is engineered to process massive multi-FASTA files and full bacterial genomes (such as *E. coli* K-12) entirely on local hardware, bypassing the need for cloud infrastructure or external APIs.

---

## Why Use Helix-rs?

* **Data Privacy & Security:** By executing entirely on your local machine, sensitive genomic data never leaves your hard drive.
* **Extreme Performance:** Utilizing Rust's memory safety and zero-cost abstractions, combined with sliding-window `O(n)` algorithms, the engine processes millions of base pairs in seconds.
* **Non-Blocking Architecture:** Built on the `tokio` asynchronous runtime, the interface remains highly responsive and provides granular progress reporting even during heavy computational workloads.
* **Comprehensive Profiling:** Natively calculates exact GC content, identifies Open Reading Frames (ORFs) across all six reading frames, and performs in-memory DNA-to-Protein translation.

---

## Prerequisites

Before downloading and installing Helix-rs, ensure your system meets the following requirements:

1. **Rust Toolchain:** You must have Cargo and `rustc` installed. 
   * Install via [rustup.rs](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Python 3 (Optional):** Required only if you intend to use the included data visualization script.
   * Required Python libraries: `pip install pandas matplotlib seaborn`

---

## Download and Installation

Follow these steps to download the source code and compile the optimized binary on your machine.

**1. Clone the Repository:**
```bash
git clone [https://github.com/ekantkannam/helix-rs.git](https://github.com/ekantkannam/helix-rs.git)
cd helix-rs

Compile the Application:
Note: You must compile using the release flag. Compiling in debug mode is not recommended for analyzing full bacterial genomes due to performance limitations.

Bash
cargo build --release
User Manual
Helix-rs is operated entirely via keyboard commands within your terminal environment.

Launching the Application

To start the engine, run the following command from the root of the project directory:

Bash
cargo run --release
Operation Controls

Once the TUI has initialized, use the following commands to navigate and control the analysis:

1 or l : Load Sequence - Opens the file input dialogue. Provide the absolute system path to your .fasta or .fna file (e.g., /Users/username/e_coli.fasta) and press Enter.

r : Run Analysis - Executes the genomic profiling engine on the currently selected sequence. The UI will display real-time progress.

t : Translate - Toggles the DNA-to-Protein translation view for the identified sequences.

e : Export Data - Writes the computed analysis metrics (Length, GC Content, ORF Count) to a local helix_export.tsv file in the project directory.

d : Load Demos - Automatically loads included demonstration sequences (e.g., HIV-1, Ebola VP40) for software testing.

UP / DOWN Arrows : Navigate - Moves the selection cursor between different sequences if a multi-FASTA file is loaded.

q : Quit - Safely terminates the application and returns to the standard terminal prompt.

Data Visualization
Helix-rs includes a companion Python script to generate scientific visualizations from your exported data.

Ensure you have analyzed your sequences and pressed e to generate the helix_export.tsv file.

Execute the visualization script:

Bash
python3 analyze_results.py
The script will read the TSV file and generate a scatter plot detailing the correlation between GC Content (%) and the Number of ORFs detected across the dataset.

System Architecture
src/engine.rs: The core computational logic. Implements sliding window algorithms for high-throughput sequence parsing.

src/tasks.rs: Background worker configurations utilizing tokio to manage thread-safe I/O operations and computation.

src/ui.rs: The Ratatui-based frontend rendering logic.

src/state.rs: Application state management and strict type definitions for biological data (ORFs, Sequence Profiles).

Developed and maintained by Ekant Kumar Kannam
