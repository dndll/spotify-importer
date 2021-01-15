use std::{fs::File, io, path::PathBuf};

use crate::{cli::Opts, provider::StreamingProvider};
use anyhow::Error;
use async_trait::async_trait;
use csv::Reader;
use serde::Deserialize;


pub struct Raw {
    queries: Vec<RawRecord>
}
impl Raw {
    fn new_from_records(records: Vec<RawRecord>) -> Raw {
        Raw {
            queries: records
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawRecord {
    artist: String,
    track: String
}

#[derive(Default, Debug, Clone)]
pub struct RawProvider {
    pub playlist: String,
    pub file: PathBuf,
}


impl RawProvider {
    pub fn new(opts: &Opts) -> RawProvider {
        RawProvider {
            playlist: opts.playlist.to_owned(),
            file: opts.raw_file.as_ref().expect("Failed to unwrap the raw_file parameter").to_path_buf(),
        }
    }
}


#[async_trait]
impl StreamingProvider<Raw> for RawProvider {

    //TODO should be response dto from reading a csv of raws
    async fn gather_data(&self) -> Result<Raw, anyhow::Error> {
        println!("> Reading csv file..");
        let raw = get_raws_from_file(&self.file)?;
        // read a source
        println!("> Importing {} tracks..", raw.queries.len());
        Ok(raw)
    }

    fn convert_to_query(&self, item: Raw) -> Vec<(String, String)> {
        item.queries.iter().map(|query| (query.artist.to_lowercase(), query.track.to_lowercase())).collect()
    }

    async fn build_queries(&self) -> Result<Vec<(String, String)>, anyhow::Error> {
        let provider = self.gather_data().await?;
        let queries = self.convert_to_query(provider);
        Ok(queries)
    }
}

pub fn get_raws_from_file(path: &PathBuf) -> Result<Raw, Error> {
    let mut rdr = Reader::from_path(path)?;
    let mut raw_records: Vec<RawRecord> = vec![];
    // TODO further rayon ops maybe
    for result in rdr.deserialize() {
        raw_records.push(result?);
    }   
    Ok(Raw::new_from_records(raw_records))
}