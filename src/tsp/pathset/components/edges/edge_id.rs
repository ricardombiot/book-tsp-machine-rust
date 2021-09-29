use crate::tsp::pathset::components::nodes::node_id::NodeId;
use std::collections::HashSet;
use std::fmt;

pub struct EdgeId {
    origin_id : NodeId,
    destine_id : NodeId
}
pub type EdgesIdSet = HashSet<NodeId> ;

impl EdgeId {
    pub fn new(origin_id: NodeId, destine_id: NodeId) -> Self { Self { origin_id, destine_id } }

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