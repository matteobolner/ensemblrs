use std::fmt::Debug;

use serde::{Deserialize, Serialize};

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
    fn to_bed(&self, name: &str) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.seq_region_name, self.coords.start, self.coords.end, name, &self.strand
        )
    }
}

// MetaInfo struct for common metadata fields
#[derive(Clone, Debug, Deserialize, Serialize)]
struct MetaInfo {
    id: String,
    db_type: String,
    //object_type: String,
    species: String,
    version: u64,
}

//DEFINE ENSEMBL FEATURES

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    fn to_bed(&self, name: &str) -> String {
        format!("{}\t{}\t{}", self.coords.start, self.coords.end, name)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Exon {
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gene {
    canonical_transcript: String,
    description: String,
    #[serde(rename = "Transcript")]
    transcripts: Option<Vec<Transcript>>,
    #[serde(flatten)]
    coords: GenomeCoords,
    #[serde(flatten)]
    meta: MetaInfo,
    display_name: String,
    logic_name: String,
    biotype: String,
    source: String,
}

impl Gene {
    pub fn get_canonical_transcript(&self) -> Option<Transcript> {
        match &self.transcripts {
            Some(transcripts) => transcripts.iter().find(|s| s.is_canonical == 1).cloned(),
            _ => None,
        }
    }

    pub fn gene_with_canonical_transcript_only(self) -> Gene {
        Gene {
            transcripts: vec![self.get_canonical_transcript().unwrap()].into(),
            ..self.clone()
        }
    }
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
    pub fn to_bed(&self) -> String {
        match self {
            EnsemblFeature::Gene(gene) => gene.coords.clone().to_bed(&gene.display_name),
            EnsemblFeature::Transcript(transcript) => {
                transcript.coords.clone().to_bed(&transcript.display_name)
            }
            EnsemblFeature::Exon(exon) => exon.coords.clone().to_bed(&exon.meta.id),
            EnsemblFeature::Protein(protein) => protein.to_bed(&protein.meta.id),
        }
    }
    /*pub fn to_tsv(&self) -> String {
        match self {
            EnsemblFeature::Gene(gene) => gene.coords.clone(),
            EnsemblFeature::Transcript(transcript) => transcript.coords.clone().row()[0].clone(),
            EnsemblFeature::Exon(exon) => exon.coords.clone().row()[0].clone(),
            EnsemblFeature::Protein(protein) => protein.coords.header()[0].clone(),
        }
    }*/
}

trait ToTsv {
    fn header(&self) -> Vec<String>;
    fn row(&self) -> Vec<String>;
}

impl ToTsv for StartEndCoords {
    fn header(&self) -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
    }
    fn row(&self) -> Vec<String> {
        vec![self.start.to_string(), self.end.to_string()]
    }
}

impl ToTsv for GenomeCoords {
    fn header(&self) -> Vec<String> {
        let mut header = vec!["assembly_name".to_string(), "seq_region_name".to_string()];
        header.extend(self.coords.header());
        header.extend(vec!["strand".to_string()]);
        header
    }
    fn row(&self) -> Vec<String> {
        let mut row = vec![
            self.assembly_name.to_string(),
            self.seq_region_name.to_string(),
        ];
        row.extend(self.coords.row());
        row.extend(vec![self.strand.to_string()]);
        row
    }
}

impl ToTsv for MetaInfo {
    fn header(&self) -> Vec<String> {
        let header = vec![
            "id".to_string(),
            "species".to_string(),
            "version".to_string(),
            "db_type".to_string(),
        ];
        header
    }
    fn row(&self) -> Vec<String> {
        let row = vec![
            self.id.clone(),
            self.species.clone(),
            self.version.to_string(),
            self.db_type.clone(),
        ];
        row
    }
}

impl ToTsv for Protein {
    fn header(&self) -> Vec<String> {
        let mut header = vec!["parent".to_string(), "length".to_string()];
        let mut meta_info_header = self.meta.header();
        let mut coords_header = self.coords.header();
        header.append(&mut meta_info_header);
        header.append(&mut coords_header);
        header
    }
    fn row(&self) -> Vec<String> {
        let mut row = vec![self.parent.clone(), self.length.to_string()];
        let mut meta_info_row = self.meta.row();
        let mut coords_row = self.coords.row();
        row.append(&mut meta_info_row);
        row.append(&mut coords_row);
        row
    }
}
impl std::fmt::Display for StartEndCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start\tend\n{}\t{}", self.start, self.end)
    }
}

//fn header(&self) -> Vec<String> {
