use clap::Parser;
pub mod lookup_id;
pub mod structs;
//use lookup_id::lookup;
pub use crate::lookup_id::lookup::lookup_id;
//"Expand option only available for Genes and Transcripts"

/// Simple program to download Ensembl Feature data from an ID
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Ensembl ID
    #[arg(required = true, short, long)]
    id: String,
    /// Expand
    #[arg(short, long)]
    expand: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let feature = lookup_id(&args.id, args.expand);
    println!("{:?}", feature.await.expect("PROVA"))
}
