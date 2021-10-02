
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::inmutable_dict::{DictInmutableWapper, InmutableDict, InmutableDictCommons};

#[derive(Clone)]
pub struct TableNodes {
    table : InmutableDict<NodeId, Node>
}

impl TableNodes {
    pub fn new() -> Self {
        let table = InmutableDict::new();
        TableNodes{table}
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
