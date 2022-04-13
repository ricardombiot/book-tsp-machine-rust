
use crate::tsp::utils::alias::{ActionId};
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
use std::fmt::Debug;



#[derive(Clone, Debug)]
pub struct TableController {
    table : InmutableDict<ActionId, u32>
}

impl TableController {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableController{table}
    }
}


impl InmutableDictCommons<ActionId, u32> for TableController {
    fn dict(&self) -> & InmutableDict<ActionId, u32>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<ActionId, u32>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<ActionId, u32>  {
        &mut self.table
    }

    fn join_item(_cells : &mut u32, _cells_join: &u32) {
       panic!("join should not use it");
    }
}