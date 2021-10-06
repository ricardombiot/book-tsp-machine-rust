use crate::tsp::utils::alias::{Color};
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
use crate::tsp::machine::components::timeline::timeline_cell::TimelineCell;

#[derive(Clone, Debug)]
pub struct TableCellsByColor {
    cells : InmutableDict<Color, TimelineCell>
}


impl TableCellsByColor {
    pub fn new() -> Self{
        let cells = InmutableDict::new();
        TableCellsByColor{cells}
    }
}


impl InmutableDictCommons<Color, TimelineCell> for TableCellsByColor {
    fn dict(&self) -> & InmutableDict<Color, TimelineCell>  {
        &self.cells
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Color, TimelineCell>  {
        &mut self.cells
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<Color, TimelineCell>  {
        &mut self.cells
    }

    fn join_item(_cells : &mut TimelineCell, _cells_join: &TimelineCell) {
    }
}