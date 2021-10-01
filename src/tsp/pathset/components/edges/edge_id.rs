use crate::tsp::pathset::components::nodes::node_id::NodeId;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug,PartialEq, Eq,Clone)]
pub struct EdgeId {
    origin_id : NodeId,
    destine_id : NodeId
}
pub type EdgesIdSet = HashSet<NodeId> ;

impl EdgeId {
    pub fn new(origin_id: &NodeId, destine_id: &NodeId) -> Self { 
        let origin_id = origin_id.clone();
        let destine_id = destine_id.clone();
        Self { origin_id, destine_id } 
    }

    /// Get a reference to the edge id's origin id.
    pub fn origin_id(&self) -> &NodeId {
        &self.origin_id
    }

    /// Get a reference to the edge id's destine id.
    pub fn destine_id(&self) -> &NodeId {
        &self.destine_id
    }
}



impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}-->{}", self.origin_id(), self.destine_id());
    }
}



impl Hash for EdgeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.origin_id.hash(state);
        self.destine_id.hash(state);
    }
}
