use arrow::datatypes::{Schema, SchemaRef};
use arrow::record_batch::RecordBatch;
use crate::datasource::DataSource;

pub struct InMemoryDataSource {
    schema: Schema,
    data: Vec<RecordBatch>,
}

impl DataSource for InMemoryDataSource {
    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn scan(&self, projection: Vec<String>) -> Box<dyn Iterator<Item=RecordBatch>> {
        let projection_indices = projection.iter().map(|name| self.schema.index_of(name)).collect();
        Box::new(self.data.iter().map(|batch| batch.project(projection_indices).unwrap()))
    }
}
