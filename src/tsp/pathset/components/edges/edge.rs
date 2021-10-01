use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use std::fmt;

#[derive(Debug,PartialEq, Eq,Clone)]
pub struct Edge {
    id : EdgeId
}

impl Edge {
    pub fn new(origin_id : &NodeId, destine_id : &NodeId) -> Edge {
        let id : EdgeId = EdgeId::new(origin_id, destine_id);
        Edge{id}
    }

    /// Get a reference to the edge's id.
    pub fn id(&self) -> &EdgeId {
        &self.id
    }

    pub fn build() {
        //@TODO
    }

    pub fn destroy() {
        //@TODO
    }
}


impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Edge({})", self.id);
    }
}

/*
    mutable struct Edge
        id :: EdgeId
    end

    function new(origin_id :: NodeId, destine_id :: NodeId) :: Edge
        id = EdgeIdentity.new(origin_id, destine_id)
        Edge(id)
    end

    function build!(node_origin :: Node, node_destine :: Node) :: Edge
        PathNode.add_son!(node_origin, node_destine)
        PathNode.add_parent!(node_destine, node_origin)

        return new(node_origin.id, node_destine.id)
    end

    function destroy!(node_origin :: Node, node_destine :: Node)
        PathNode.delete_son!(node_origin, node_destine)
        PathNode.delete_parent!(node_destine, node_origin)
    end
*/