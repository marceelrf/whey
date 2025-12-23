use bio::io::fasta;
use std::path::Path;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct FastaRecord {
    pub id: String,
    // pub desc: Option<String>,
    pub seq: Vec<u8>,
}

pub fn read_fasta<P>(
    path: P,
) -> Result<Vec<FastaRecord>, Box<dyn Error>>
where
    P: AsRef<Path> + Debug,
{
    let reader = fasta::Reader::from_file(path)?;

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;

        records.push(FastaRecord {
            id: record.id().to_string(),
            // desc: record.desc().map(|d| d.to_string()),
            seq: record.seq().to_vec(),
        });
    }

    Ok(records)
}