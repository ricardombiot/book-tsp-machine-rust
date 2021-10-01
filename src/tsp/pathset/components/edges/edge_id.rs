use crate::tsp::pathset::components::nodes::node_id::NodeId;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug,PartialEq, Eq,Clone)]
pub struct EdgeId<'a,'b> {
    origin_id : &'a NodeId,
    destine_id : &'b NodeId
}
pub type EdgesIdSet = HashSet<NodeId> ;

impl<'a,'b> EdgeId<'a,'b> {
    pub fn new(origin_id: &'a NodeId, destine_id: &'b NodeId) -> Self { Self { origin_id, destine_id } }

    /// Get a reference to the edge id's origin id.
    pub fn origin_id(&self) -> &'a NodeId {
        &self.origin_id
    }

    /// Get a reference to the edge id's destine id.
    pub fn destine_id(&self) -> &'b NodeId {
        &self.destine_id
    }
}



impl<'a,'b> fmt::Display for EdgeId<'a,'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}-->{}", self.origin_id(), self.destine_id());
    }
}