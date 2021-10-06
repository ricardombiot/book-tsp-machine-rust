
use crate::tsp::utils::alias::{Color, Km, Step};
use crate::tsp::utils::generator_ids;
use crate::tsp::utils::inmutable_dict::{InmutableDictCommons,InmutableDict};
use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::utils::alias::ActionId;
pub mod table_cells_by_color;
pub mod timeline_cell;

use crate::tsp::machine::components::timeline::table_cells_by_color::TableCellsByColor;

use self::timeline_cell::TimelineCell;

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

    pub fn have_cell(&self, km : &Km, color : &Color) -> bool {
        if self.have(&km) {
            return self.get(&km).unwrap().have(&color);
        }else{
            return false;
        }
    }

    pub fn get_cell<'user>(&'user self, km : &'user Km, color : &'user Color) -> Option<&'user TimelineCell>{
        if self.have(km) {
            return self.get(km).unwrap().get(color);
        }else{
            return None;
        }
    }

    pub fn get_mut_cell<'user>(&'user mut self, km : &'user Km, color : &'user Color) -> Option<&'user mut TimelineCell>{
        if self.have(km) {
            return self.get_mut(km).unwrap().get_mut(color);
        }else{
            return None;
        }
    }

    pub fn create_cell(&mut self, km : &Km, color : &Color, action_id : Option<ActionId>){
        if !self.have(km) {
            let table_cells_by_color = TableCellsByColor::new();
            self.put(km.clone(), table_cells_by_color);
        }

        let table_cells_by_color = self.get_mut(&km).unwrap();
        if ! table_cells_by_color.have(color) {
            let new_cell = TimelineCell::new(km.clone(),color.clone(), action_id);
            table_cells_by_color.put(color.clone(), new_cell)
        }
    }

    pub fn put_init(&mut self, n : &Color, color : &Color){
        let km : Km = 0;
        let action_id = generator_ids::get_action_id(n.clone(), km.clone(), color.clone());
        self.create_cell(&km, &color, Some(action_id));
    }

    pub fn push_parent(&mut self, km : &Km, color : &Color, parent_id : &ActionId){
        if !self.have_cell(km, color) {
            self.create_cell(km, color, None);
        }

        let cell: &mut TimelineCell = self.get_mut_cell(km, color).unwrap();
        cell.push_parent(parent_id.clone());
    }

    pub fn execute(&mut self, db : &mut DatabaseActions, km : Km, color: Color) -> (bool, Option<ActionId>) {
        if self.have_cell(&km, &color) {
            let cell = self.get_mut_cell(&km, &color).unwrap();
            return cell.execute(db);
        }

        return (false, None);
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