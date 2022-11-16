use std::path::{Path, PathBuf};
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use crate::datasource::DataSource;

pub struct ParquetDataSource {
    filename: PathBuf,
}

impl ParquetDataSource {
    pub fn new(filename: PathBuf) -> Self {
        Self { filename }
    }
}

impl DataSource for ParquetDataSource {
    fn schema(&self) -> &Schema {
        todo!()
    }

    fn scan(&self, projection: Vec<String>) -> Box<dyn Iterator<Item=RecordBatch>> {
        todo!()
    }
}
