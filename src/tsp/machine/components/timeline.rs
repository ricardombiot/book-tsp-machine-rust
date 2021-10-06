
use crate::tsp::utils::alias::{Color, Km};
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
pub mod table_cells_by_color;
pub mod timeline_cell;

use crate::tsp::machine::components::timeline::table_cells_by_color::TableCellsByColor;

#[derive(Clone, Debug)]
pub struct Timeline {
    table_cells : InmutableDict<Km, TableCellsByColor>
}


impl InmutableDictCommons<Km, TableCellsByColor> for Timeline {
    fn dict(&self) -> & InmutableDict<Km, TableCellsByColor>  {
        &self.table_cells
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Km, TableCellsByColor>  {
        &mut self.table_cells
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<Km, TableCellsByColor>  {
        &mut self.table_cells
    }

    fn join_item(table_cells : &mut TableCellsByColor, table_cells_join: &TableCellsByColor) {
    }
}