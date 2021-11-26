use std::sync::Arc;

use futures::SinkExt;

use crate::{DocAddress, SegmentOrdinal, schema::Field, vector::VectorField};

use super::{Collector, SegmentCollector};

/// Collector for vectors
///
/// The collector collects all vectors though all segments.
pub struct VectorCollector {
    field: Field
}

impl Collector for VectorCollector {
    type Fruit = Vec<(DocAddress, Vec<f32>)>;
    type Child = VectorSegmentCollector;

    fn for_segment(
        &self,
        segment_local_id: crate::SegmentOrdinal,
        reader: &crate::SegmentReader,
    ) -> crate::Result<Self::Child> {
        trace!("for_segment");
        trace!("Segment local id: {}", segment_local_id);

        let vector_reader = reader.vector_reader(self.field)?;

        Ok(VectorSegmentCollector::new(vector_reader, segment_local_id))
    }

    fn requires_scoring(&self) -> bool {
        true
    }

    fn merge_fruits(
        &self,
        segment_fruits: Vec<<Self::Child as super::SegmentCollector>::Fruit>,
    ) -> crate::Result<Self::Fruit> {
        // Vec<(crate::DocId, Vec<f32>)>
        // TODO: Add HeapMap?
        let mut fruits = Vec::new();

        for fruit in segment_fruits {
            for s in fruit {
                fruits.push(s);
            }
        }
        Ok(fruits)
    }

    
}

impl VectorCollector {
    pub fn for_field(field: Field) -> VectorCollector {
        VectorCollector { field }
    }
}

pub struct VectorSegmentCollector {
    vector_segment: Arc<VectorField>,
    fruits: Vec<(DocAddress, Vec<f32>)>,
    segment_ord: u32,
}

impl VectorSegmentCollector {
    fn new(vector_segment: Arc<VectorField>, segment_ord: SegmentOrdinal) -> VectorSegmentCollector {
        VectorSegmentCollector {
            vector_segment,
            fruits: Vec::new(),
            segment_ord
        }
    }
}

impl SegmentCollector for VectorSegmentCollector {
    type Fruit = Vec<(DocAddress, Vec<f32>)>;

    fn collect(&mut self, doc_id: crate::DocId, score: crate::Score) {

        debug!("Calling collect on docId: {} score: {}", doc_id, score);

        let doc_addr = DocAddress{
            segment_ord: self.segment_ord,
            doc_id: doc_id
        };

        self.fruits.push((doc_addr, vec![0.0,1.0,2.0,3.0]))

    }

    fn harvest(self) -> Self::Fruit {
        debug!("Harvest!");
        return self.fruits;
    }
}

