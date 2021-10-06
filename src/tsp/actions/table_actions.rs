
use crate::tsp::utils::alias::{ActionId};
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
use crate::tsp::actions::action::Action;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct TableActions {
    table : InmutableDict<ActionId, Action>
}

impl TableActions {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableActions{table}
    }
}


impl InmutableDictCommons<ActionId, Action> for TableActions {
    fn dict(&self) -> & InmutableDict<ActionId, Action>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<ActionId, Action>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<ActionId, Action>  {
        &mut self.table
    }

    fn join_item(_cells : &mut Action, _cells_join: &Action) {
       panic!("join should not use it");
    }
}