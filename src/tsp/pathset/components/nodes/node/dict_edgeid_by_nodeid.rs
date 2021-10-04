use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::utils::inmutable_dict::{InmutableDict,InmutableDictCommons};

#[derive(Clone)]
pub struct DictEdgeIdByNodeId {
    dict : InmutableDict<NodeId, EdgeId>
}


impl DictEdgeIdByNodeId {
    pub fn new() -> Self{
        let dict : InmutableDict<NodeId, EdgeId> = InmutableDict::new();
        DictEdgeIdByNodeId{dict}
    }

    pub fn add_edge_id(&mut self, origin_id: &NodeId, destine_id: &NodeId, key : &NodeId){
        let edge_id = EdgeId::new(origin_id, destine_id);
        self.put(key.clone(), edge_id);
    }   
}


impl InmutableDictCommons<NodeId, EdgeId> for DictEdgeIdByNodeId {

    fn dict(&self) -> & InmutableDict<NodeId, EdgeId>  {
        &self.dict
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<NodeId, EdgeId>  {
        &mut self.dict
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<NodeId, EdgeId> {
        &mut self.dict   
    }

    fn join_item(_original : &mut EdgeId, _other : &EdgeId) {
    }

}