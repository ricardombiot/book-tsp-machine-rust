use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DictEdgeIdByNodeId {
    dict : HashMap<NodeId, EdgeId>
}

impl DictEdgeIdByNodeId {
    pub fn new() -> Self{
        let dict : HashMap<NodeId, EdgeId> = HashMap::new();
        DictEdgeIdByNodeId{dict}
    }

    pub fn for_each<F>(&mut self, mut func : F) where F : FnMut(&NodeId, &EdgeId) {
        for (node_id, edge_id) in self.dict.iter() {
            func(node_id, edge_id)
        }
    }

    pub fn add(&mut self, origin_id: &NodeId, destine_id: &NodeId, key : &NodeId){
        let edge_id = EdgeId::new(origin_id, destine_id);
        self.dict.insert(key.clone(), edge_id);
    }

    pub fn have(&self, key : &NodeId) -> bool {
        self.dict.contains_key(key)
    }

    pub fn delete(&mut self, key : &NodeId){
        self.dict.remove(key);
    }

    pub fn is_empty(&self)-> bool {
        self.dict.is_empty()
    }

}