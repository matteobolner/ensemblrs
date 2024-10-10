use clap::{Args, Parser, Subcommand};
pub mod lookup_id;
pub mod structs;
//use lookup_id::lookup;
pub use crate::lookup_id::lookup::lookup_id;
//"Expand option only available for Genes and Transcripts"

/// Simple program to download Ensembl Feature data from an ID

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lookup
    Lookup(Lookup),
}

#[derive(Args)]
struct Lookup {
    /// Ensembl ID
    #[arg(required = true, short, long)]
    id: String,
    /// Expand
    #[arg(required = false, short, long, default_value_t = String::from("false"))]
    expand: String,
    /// Expand
    #[arg(required = false, short, long, default_value_t = String::from("bed"))]
    output: String,
}
/*
struct Args {
    /// Ensembl ID
    #[arg(required = true, short, long)]
    id: String,
    /// Expand
    #[arg(required = false, short, long, default_value_t = String::from("false"))]
    expand: String,
}
*/

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Lookup(Lookup { id, expand, output }) => {
            let feature = lookup_id(id, expand).await.expect("ERROR");
            match output.as_str() {
                "bed" => println!("{}", feature.to_bed()),
                _ => eprintln!("Unsupported output format: {}", output),
            }
        }
    }
}
