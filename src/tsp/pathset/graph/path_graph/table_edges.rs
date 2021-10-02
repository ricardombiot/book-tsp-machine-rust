use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::edges::edge::Edge;
use crate::tsp::utils::inmutable_dict::{InmutableDict,InmutableDictCommons};

#[derive(Clone)]
pub struct TableEdges {
    table : InmutableDict<EdgeId, Edge>
}

impl TableEdges {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableEdges{table}
    }
}

impl InmutableDictCommons<EdgeId, Edge> for TableEdges {
    fn dict(&self) -> & InmutableDict<EdgeId, Edge>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<EdgeId, Edge>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<EdgeId, Edge>  {
        &mut self.table
    }
}
