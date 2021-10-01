
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::inmutable_dict::{InmutableDict,InmutableDictCommons};

#[derive(Clone)]
pub struct TableNodes {
    table : InmutableDict<NodeId, Node>
}

impl TableNodes {
    pub fn new() -> Self {
        let table = InmutableDict::new();
        TableNodes{table}
    }
}

impl InmutableDictCommons<NodeId, Node> for TableNodes {
    fn dict(&self) -> & InmutableDict<NodeId, Node>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<NodeId, Node>  {
        &mut self.table
    }
}
