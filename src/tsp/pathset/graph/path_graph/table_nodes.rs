
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::inmutable_dict::{InmutableDict, InmutableDictCommons};

#[derive(Clone)]
pub struct TableNodes {
    table : InmutableDict<NodeId, Node>
}

impl TableNodes {
    pub fn new() -> Self {
        let table = InmutableDict::new();
        TableNodes{table}
    }

    pub(super) fn _push_node_as_new_owner(&mut self, node_id: &NodeId){
        let list_nodes_id = self.table.to_list_keys();
        for current_node_id in list_nodes_id.iter() {
            let node = self.get_mut(current_node_id).unwrap();
            node.push_owner(node_id);
        }
    }

    pub(super) fn delete_node(&mut self, node_id : &NodeId){
        if self.have(node_id){
            self.delete(node_id);
        }
    }

    pub fn apply_node<F,R>(&self,node_id : & NodeId, mut func: F) -> Result<R,String> 
        where F : FnMut(&Node) -> R {
            match self.get(node_id) {
                None => return Err("not_found_node_id".to_string()),
                Some(node) =>  { 
                    let result : R = func(node);
                    return Ok(result)
                }
            }
    }

    pub fn apply_mut_node<F,R>(&mut self,node_id : &NodeId, mut func: F) -> Result<R,String> 
    where F : FnMut(&mut Node) -> R {
        match self.get_mut(node_id) {
            None => return Err("not_found_node_id".to_string()),
            Some(node) => {
                let result : R = func(node);
                return Ok(result)
            }    
        }
    }

    pub fn add_node(&mut self, node : Node){
        self._put_node(node);
    }

    fn _put_node(&mut self, node : Node){
        self.put(node.id().clone(), node);
    }

    fn get_node_mut<'user>(&'user mut self, node_id : &'user NodeId) -> Option<&'user mut Node>{
        self.get_mut(node_id)
    }
}

impl InmutableDictCommons<NodeId, Node> for TableNodes {
    fn dict(&self) -> & InmutableDict<NodeId, Node>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<NodeId, Node>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<NodeId, Node>  {
        &mut self.table
    }
}
