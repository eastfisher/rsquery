use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use arrow::csv;
use arrow::csv::Reader;

use crate::datasource::DataSource;

pub struct CsvDataSource {
    filepath: PathBuf,
    csv: Reader<File>, // TODO: lazy init
}

impl CsvDataSource {
    pub fn open(filepath: PathBuf) -> Result<Self> {
        let f = File::open(filepath.as_path())?;
        let builder = csv::ReaderBuilder::new()
            .has_header(true)
            .infer_schema(Some(100));
        let csv = builder.build(f)?;
        Ok(Self { filepath, csv })
    }
}

impl DataSource for CsvDataSource {
    fn schema(&self) -> &Schema {
        self.csv.schema().as_ref()
    }

    fn scan(&self, projection: Vec<String>) -> Box<dyn Iterator<Item=RecordBatch>> {
        let projection_indices = projection.iter().map(|name| self.schema.index_of(name)).collect();
        Box::new(self.csv
            .map(|batch| batch.unwrap())
            .map(|batch| batch.project(projection_indices).unwrap())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan() -> Result<()> {
        let path = PathBuf::from("../testdata/employee.csv");
        let ds = CsvDataSource::open(path)?;
        let headers = vec!["id", "first_name", "last_name", "state", "job_title", "salary"];
        assert_eq!(headers.len() ,ds.schema().fields().len());
        Ok(())
    }
}
