use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use qdrant_segment::{entry::entry_point::SegmentEntry, segment_constructor::{build_segment, load_segment}, types::{Distance, Indexes, SegmentConfig, WithPayload}};

use crate::{DocId, TantivyError, schema::{Field, Schema}};



type ScoreType = f32;

pub struct VectorField {
    //segment: qdrant_segment::segment::Segment,
    segment: u32
}

impl VectorField {

    /// Creates a VectorReader on this path. Usually this method is call from the VectorReaders
    /// container of the segment reader.
    pub fn new(segment_path: &PathBuf, field: Field, config: &SegmentConfig) -> VectorField {
        
        let field_path = field.field_id().to_string();
        let path = segment_path.join(field_path);

        trace!("New VectorWriter for field: {:?} at {}", field, path.to_str().unwrap());

        let segment = match load_segment(path.as_path()) {
            Ok(segment) => segment,
            Err(e) => match build_segment(path.as_path(), &config) {
                Ok(segment) => segment,
                Err(e) => {
                    panic!("Error loading VectorWriter: {}", e);
                }
            },
        };

        VectorField { segment: 2 }
    }

    /// Search documents with similarity to this vector.
    pub fn search(&self, vector: &Vec<f32>, limit: usize) -> Vec<(DocId, ScoreType)> {
        /*
        let res = self
            .segment
            .search(&vector, &WithPayload::default(), None, limit, None)
            .unwrap();

        res.iter().map(|x | {
            (x.id as DocId, x.score as ScoreType)
        }).collect()
        */
        todo!();
    }


    /// Stores vector for this document
    pub fn record(&self, doc_id: DocId, vector: &Vec<f32>) -> crate::Result<bool> {
        trace!("record => {} - {:?}", doc_id, vector);
        /*
        match self.segment.upsert_point(1, doc_id as u64, vector) {
            Ok(b) => Ok(b),
            Err(e) => Err(TantivyError::InvalidArgument(e.to_string())),
        }
        */
        todo!()
    }
}
