
use crate::tsp::utils::alias::{Color, Km, Step};
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
pub mod table_cells_by_color;
pub mod timeline_cell;

use crate::tsp::machine::components::timeline::table_cells_by_color::TableCellsByColor;

#[derive(Clone, Debug)]
pub struct Timeline {
    table_cells : InmutableDict<Km, TableCellsByColor>
}


impl Timeline {
    pub fn new() -> Self{
        let table_cells = InmutableDict::new();
        Timeline{table_cells}
    }

    pub fn get_mut_line<'user>(&'user mut self, line : &'user Step) -> Option<&'user mut TableCellsByColor>{
        return self.get_mut(line);
    }
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
        todo!();
    }
}