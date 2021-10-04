
use crate::tsp::utils::alias::ActionId;
use crate::tsp::utils::inmutable_dict::{DictInmutableWapper, InmutableDict, InmutableDictCommons};
use crate::tsp::pathset::graph::path_graph::table_nodes::TableNodes;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct TableNodesByAction {
    table : InmutableDict<ActionId, TableNodes>
}

impl TableNodesByAction {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableNodesByAction{table}
    }

    /*
    pub fn join(&mut self, table_to_join : &TableNodesByAction){
        let list_actions_id_to_join = table_to_join.dict().to_list_keys() ;

        for action_id in list_actions_id_to_join {
            let table_nodes_to_join = table_to_join.get(&action_id).unwrap();
            let table_nodes = self.table.get_mut(&action_id);
            match table_nodes {
                None => self.put(action_id.clone(), table_nodes_to_join.clone()),
                Some(table_nodes) => table_nodes.join(&table_nodes_to_join)
            }
        }
    }*/

    pub(super) fn delete_node(&mut self, node_id : &NodeId){
        let action_id = node_id.action_id();
        let table_nodes = self.table.get_mut(&action_id);
        match table_nodes {
            None => (),
            Some(table_nodes) => {
                table_nodes.delete_node(node_id);

                if table_nodes.is_empty() {
                    self.delete(&action_id);
                }
            }
        } 
    }

    pub fn have_node(&self, node_id : & NodeId) -> bool {
        let action_id = node_id.action_id();
        let table_nodes = self.table.get(&action_id);
        match table_nodes {
            None => false,
            Some(table_nodes) => {
                return table_nodes.have(node_id)
            }
        }   
    }

    pub fn apply_node<F,R>(&self, action_id : & ActionId, node_id : & NodeId, func: F) -> Result<R,String> 
        where F : FnMut(&Node) -> R {
            match self.get(action_id) {
                None => return Err("not_found_action_id".to_string()),
                Some(table_nodes) => {
                    return table_nodes.apply_node(node_id, func);
                }
            } 
    }

    pub fn apply_mut_node<F,R>(&mut self, action_id : & ActionId, node_id : & NodeId, func: F) -> Result<R,String> 
    where F : FnMut(&mut Node) -> R {
        match self.get_mut(action_id) {
            None => return Err("not_found_action_id".to_string()),
            Some(table_nodes) => {
                return table_nodes.apply_mut_node(node_id, func);
            }
        } 
    }



    pub fn add_node(&mut self, node : Node){
        let action_id : ActionId = node.action_id().clone();
        self._if_not_exist_init_action(action_id);

        let table_nodes = self.table.get_mut(&action_id).unwrap();
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

    pub fn get_node<'user>(&'user self, action_id : &'user ActionId, node_id : &'user NodeId) -> Option<&'user Node>{
        let opt_table_nodes : Option<&'user TableNodes> = self.table.get(action_id);
        match opt_table_nodes {
            None => return None,
            Some(table_nodes) => {
                let table_nodes : &'user TableNodes = table_nodes;
                return table_nodes.get(node_id)
            }
        }
    }


    /*
    pub fn get_node_mut<'user>(&'user mut self, node_id : &'user NodeId) -> Option<&'user mut Node>{
        let action_id : &'user ActionId = &node_id.action_id().clone();
        let opt_table_nodes : Option<&'user mut TableNodes> = self.table.get_mut(&action_id);
        match opt_table_nodes {
            None => return None,
            Some(table_nodes) => {
                let table_nodes : &'user mut TableNodes = table_nodes;
                return table_nodes.get_mut(node_id)
            }
        }
    }

    pub fn get_node<'user>(&'user self, node_id : &'user NodeId) -> Option<&'user Node>{
        let action_id = node_id.action_id();
        let opt_table_nodes : Option<&'user TableNodes> = self.table.get(&action_id);
        match opt_table_nodes {
            None => return None,
            Some(table_nodes) => {
                let table_nodes : &'user TableNodes = table_nodes;
                return table_nodes.get(node_id)
            }
        }
    }*/

    fn _if_not_exist_init_action(&mut self, action_id : ActionId){
        let table_nodes = TableNodes::new();
        self.put(action_id , table_nodes);
    }

    pub(super) fn _push_node_as_new_owner(&mut self, node_id: &NodeId){
        let list_action_id = self.table.to_list_keys();
        for action_id in list_action_id.iter() {
            let table_nodes = self.table.get_mut(action_id).unwrap();

            table_nodes._push_node_as_new_owner(node_id);
        }
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

    fn join_item(original_table_nodes : &mut TableNodes, table_nodes_join: &TableNodes) {
        original_table_nodes.join(table_nodes_join)
    }
}
