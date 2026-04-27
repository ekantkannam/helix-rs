mod app;
mod engine;
mod fasta;
mod state;
mod tasks;
mod ui;
mod worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let fasta_path = std::env::args().nth(1);
    app::run_app(fasta_path).await
}
