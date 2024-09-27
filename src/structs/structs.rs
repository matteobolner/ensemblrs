use serde::{Deserialize, Serialize};

//"Expand option only available for Genes and Transcripts"

//DEFINE STRUCTS FOR GENERAL USE

#[derive(Debug, Deserialize, Serialize)]
struct StartEndCoords {
    start: u64,
    end: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct GenomeCoords {
    assembly_name: String,
    seq_region_name: String,
    #[serde(flatten)]
    coords: StartEndCoords,
    strand: i8,
}

// MetaInfo struct for common metadata fields
#[derive(Debug, Deserialize, Serialize)]
struct MetaInfo {
    id: String,
    db_type: String,
    object_type: String,
    species: String,
    version: u64,
}

//DEFINE ENSEMBL FEATURES

#[derive(Debug, Deserialize, Serialize)]
struct Protein {
    length: i32,
    #[serde(rename = "Parent")]
    parent: String,
    #[serde(flatten)]
    coords: StartEndCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Exon {
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transcript {
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
struct Gene {
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
