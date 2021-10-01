
use crate::tsp::utils::alias::ActionId;
use crate::tsp::utils::inmutable_dict::{InmutableDict,InmutableDictCommons};
use crate::tsp::pathset::graph::path_graph::table_nodes::TableNodes;

#[derive(Clone)]
pub struct TableNodesByAction {
    table : InmutableDict<ActionId, TableNodes>
}

impl TableNodesByAction {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableNodesByAction{table}
    }
}

impl InmutableDictCommons<ActionId, TableNodes> for TableNodesByAction {
    fn dict(&self) -> & InmutableDict<ActionId, TableNodes>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<ActionId, TableNodes>  {
        &mut self.table
    }
}
