
use crate::tsp::utils::alias::{Step};
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct TableGraphByLenght {
    table : InmutableDict<Step, PathGraph>
}

impl TableGraphByLenght {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableGraphByLenght{table}
    }

    pub fn push_graph_by_lenght(&mut self, graph_join : PathGraph){
        let lenght = graph_join.get_lenght();
        let current_graph = self.get_mut(&lenght);

        if current_graph.is_none() {
            self.put(lenght, graph_join);
        }else{
            current_graph.unwrap().join(&graph_join);
        }
    }
}


impl InmutableDictCommons<Step, PathGraph> for TableGraphByLenght {
    fn dict(&self) -> & InmutableDict<Step, PathGraph>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Step, PathGraph>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<Step, PathGraph>  {
        &mut self.table
    }

    fn join_item(_cells : &mut PathGraph, _cells_join: &PathGraph) {
       panic!("join should not use it");
    }
}