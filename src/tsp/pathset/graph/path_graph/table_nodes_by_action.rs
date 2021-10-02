
use crate::tsp::utils::alias::ActionId;
use crate::tsp::utils::inmutable_dict::{DictInmutableWapper, InmutableDict, InmutableDictCommons};
use crate::tsp::pathset::graph::path_graph::table_nodes::TableNodes;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;

#[derive(Clone)]
pub struct TableNodesByAction {
    table : InmutableDict<ActionId, TableNodes>
}

impl TableNodesByAction {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableNodesByAction{table}
    }

    pub fn add_node(&mut self, node : Node){
        let action_id : ActionId = node.action_id();
        self._if_not_exist_init_action(action_id);

        let mut table_nodes = self.table.get_mut(&action_id).unwrap();
        table_nodes.add_node(node);
        /*let mut table_nodes = self.table.pop(&action_id).unwrap();
        table_nodes.add_node(node);
        self.put(action_id , table_nodes);*/
    }

    pub fn get_node_mut<'user>(&'user mut self, action_id : &'user ActionId, node_id : &'user NodeId) -> Option<&'user mut Node>{
        let opt_table_nodes : Option<&'user mut TableNodes> = self.table.get_mut(action_id);
        match opt_table_nodes {
            None => return None,
            Some(table_nodes) => {
                let table_nodes : &'user mut TableNodes = table_nodes;
                return table_nodes.get_mut(node_id)
            }
        }
    }


    fn _if_not_exist_init_action(&mut self, action_id : ActionId){
        let table_nodes = TableNodes::new();
        self.put(action_id , table_nodes);
    }
}

impl InmutableDictCommons<ActionId, TableNodes> for TableNodesByAction {
    fn dict(&self) -> & InmutableDict<ActionId, TableNodes>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<ActionId, TableNodes>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<ActionId, TableNodes>  {
        &mut self.table
    }
}
