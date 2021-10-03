
use crate::tsp::utils::alias::{Step};
use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
use crate::tsp::utils::inmutable_dict::{DictInmutableWapper, InmutableDict,InmutableDictCommons};
use crate::tsp::pathset::components::nodes::node_id::NodeId;

#[derive(Clone)]
pub struct TableLines {
    table : InmutableDict<Step, NodesIdSet>
}

impl TableLines {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableLines{table}
    }

    pub fn add_node(&mut self, step : &Step , node_id : &NodeId){
        let set_line = self.table.get_mut(&step).unwrap();
        set_line.insert(node_id.clone());
    }

    pub fn delete_node(&mut self, step : &Step , node_id : &NodeId){
        let set_line = self.table.get_mut(&step).unwrap();
        set_line.remove(node_id);
    }

    pub fn add_line(&mut self, step: Step){
        let set_nodes = NodesIdSet::new();
        self.put(step, set_nodes);
    }
}

impl InmutableDictCommons<Step, NodesIdSet> for TableLines {
    fn dict(&self) -> & InmutableDict<Step, NodesIdSet>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Step, NodesIdSet>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<Step, NodesIdSet>  {
        &mut self.table
    }

    fn join_item(original_set : &mut NodesIdSet, join_set: &NodesIdSet) {
        for node_id in join_set.iter() {
            original_set.insert(node_id.clone());
        }
    }
}
