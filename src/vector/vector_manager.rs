use std::{collections::HashMap, path::PathBuf, sync::Arc};

use qdrant_segment::types::{Distance, Indexes, SegmentConfig};

use crate::{DocId, schema::Field, space_usage::PerFieldSpaceUsage};

use super::VectorSegment;

/// VectorWriter for segment.
/// Internally contains a Writer for each field. Lazy Initialization, a writer is created when a
/// search access its field.
pub struct VectorManager {
    segment_path: PathBuf,
    vector_map: HashMap<Field, Arc<VectorSegment>>,
    segment_config: SegmentConfig,
}

impl VectorManager {
    pub fn new(segment_path: PathBuf) -> VectorManager {
        trace!("Create VectorManager for segment");

        let config = SegmentConfig {
            vector_size: 3,
            distance: Distance::Dot,
            index: Indexes::Plain {},
            payload_index: None,
            storage_type: Default::default(),
        };

        VectorManager {
            segment_path,
            segment_config: config,
            vector_map: HashMap::new(),
        }
    }

    pub fn record(&mut self, doc_id: DocId, field: Field, vector: &Vec<f32>) -> crate::Result<bool> {
        trace!("record {} - {:?} - {:?}", doc_id, field, vector);
        match self.vector_map.get_mut(&field) {
            Some(writer) => {
                writer.record(doc_id, vector)
            }
            None => {
                let mut writer = VectorSegment::new(&self.segment_path, field, &self.segment_config);
                let result = writer.record(doc_id, vector);
                self.vector_map.insert(field, Arc::new(writer));

                result
            },
        }
        
        
    }

    pub fn open_read(&self, field: Field) -> Arc<VectorSegment> {
        match self.vector_map.get(&field) {
            Some(vector_segment) => Arc::clone(vector_segment),
            None => {
                let vector_segment = Arc::new(VectorSegment::new(&self.segment_path, field, &self.segment_config));
                self.vector_map.insert(field, vector_segment);

                vector_segment
            },
        }
    }

    /// Computes the storage needed to index this field.
    pub fn space_usage(&self) -> PerFieldSpaceUsage {
        todo!();
    }
}