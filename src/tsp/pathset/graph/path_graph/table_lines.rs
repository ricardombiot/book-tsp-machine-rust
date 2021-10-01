
use crate::tsp::utils::alias::{Step};
use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
use crate::tsp::utils::inmutable_dict::{InmutableDict,InmutableDictCommons};

#[derive(Clone)]
pub struct TableLines {
    table : InmutableDict<Step, NodesIdSet>
}

impl TableLines {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableLines{table}
    }
}

impl InmutableDictCommons<Step, NodesIdSet> for TableLines {
    fn dict(&self) -> & InmutableDict<Step, NodesIdSet>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Step, NodesIdSet>  {
        &mut self.table
    }
}
