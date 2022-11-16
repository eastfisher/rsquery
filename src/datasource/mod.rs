use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;

pub mod in_memory;
pub mod parquet;
pub mod csv;

pub trait DataSource {
    /// Return the schema for the underlying data source
    fn schema(&self) -> &Schema;

    /// Scan the data source, selecting the specified columns
    fn scan(&self, projection: Vec<String>) -> Box<dyn Iterator<Item=RecordBatch>>;
}
