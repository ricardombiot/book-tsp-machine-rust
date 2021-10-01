
use crate::tsp::utils::alias::{Color};
use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
use crate::tsp::utils::inmutable_dict::{InmutableDict,InmutableDictCommons};

#[derive(Clone)]
pub struct TableColorNodes {
    table : InmutableDict<Color, NodesIdSet>
}

impl TableColorNodes {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableColorNodes{table}
    }
}

impl InmutableDictCommons<Color, NodesIdSet> for TableColorNodes {
    fn dict(&self) -> & InmutableDict<Color, NodesIdSet>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Color, NodesIdSet>  {
        &mut self.table
    }
}
