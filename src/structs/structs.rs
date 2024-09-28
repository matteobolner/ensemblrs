use serde::{Deserialize, Serialize};
use std::io::{self, Write};

//"Expand option only available for Genes and Transcripts"

//DEFINE STRUCTS FOR GENERAL USE

#[derive(Clone, Debug, Deserialize, Serialize)]
struct StartEndCoords {
    start: u64,
    end: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GenomeCoords {
    assembly_name: String,
    seq_region_name: String,
    #[serde(flatten)]
    coords: StartEndCoords,
    strand: i8,
}

impl GenomeCoords {
    fn to_bed(&self, name: &str) -> io::Result<()> {
        let mut stdout = io::stdout(); // Get the handle to stdout
        writeln!(
            stdout,
            "{}\t{}\t{}\t{}\t{}",
            self.seq_region_name, self.coords.start, self.coords.end, name, &self.strand
        )?;
        stdout.flush()?;
        Ok(())
    }
}
// MetaInfo struct for common metadata fields
#[derive(Debug, Deserialize, Serialize)]
struct MetaInfo {
    id: String,
    db_type: String,
    //object_type: String,
    species: String,
    version: u64,
}

//DEFINE ENSEMBL FEATURES

#[derive(Debug, Deserialize, Serialize)]
pub struct Protein {
    length: i32,
    #[serde(rename = "Parent")]
    parent: String,
    #[serde(flatten)]
    coords: StartEndCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

impl Protein {
    fn to_bed(&self, name: &str) -> io::Result<()> {
        let mut stdout = io::stdout(); // Get the handle to stdout
        writeln!(
            stdout,
            "{}\t{}\t{}",
            self.coords.start, self.coords.end, name
        )?;
        stdout.flush()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exon {
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transcript {
    #[serde(rename = "Parent")]
    parent: String,
    #[serde(rename = "Exon")]
    exons: Option<Vec<Exon>>,
    #[serde(rename = "Translation")]
    translation: Option<Protein>,
    is_canonical: u64,
    length: u64,
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
    display_name: String,
    logic_name: String,
    biotype: String,
    source: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gene {
    canonical_transcript: String,
    description: String,
    #[serde(rename = "Transcript")]
    transcript: Option<Vec<Transcript>>,
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
    display_name: String,
    logic_name: String,
    biotype: String,
    source: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "object_type")]
pub enum EnsemblFeature {
    Gene(Gene),
    Transcript(Transcript),
    Exon(Exon),
    Protein(Protein),
}

// Implement to_bed for EnsemblFeature
impl EnsemblFeature {
    pub fn to_bed(&self) -> () {
        match self {
            EnsemblFeature::Gene(gene) => gene.coords.clone().to_bed(&gene.display_name).expect(""),
            EnsemblFeature::Transcript(transcript) => transcript
                .coords
                .clone()
                .to_bed(&transcript.display_name)
                .expect(""),
            EnsemblFeature::Exon(exon) => exon.coords.clone().to_bed(&exon.meta.id).expect(""),
            EnsemblFeature::Protein(protein) => protein.to_bed(&protein.meta.id).expect(""),
        }
    }
}
