use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::edges::edge::Edge;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::inmutable_dict::{InmutableDict, InmutableDictCommons};

#[derive(Clone)]
pub struct TableEdges {
    table : InmutableDict<EdgeId, Edge>
}

impl TableEdges {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableEdges{table}
    }

    pub fn build_edge(&mut self, node_parent_id: & NodeId, node_son_id: & NodeId){
        let edge = Edge::new(node_parent_id, node_son_id);
        self.put(edge.id().clone(), edge);
    }

}

impl InmutableDictCommons<EdgeId, Edge> for TableEdges {
    fn dict(&self) -> & InmutableDict<EdgeId, Edge>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<EdgeId, Edge>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<EdgeId, Edge>  {
        &mut self.table
    }

    fn join_item(original_edge : &mut Edge, join_edge: &Edge) {
    }
}
