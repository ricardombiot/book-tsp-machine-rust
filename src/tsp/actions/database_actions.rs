use std::fmt;
use crate::tsp::utils::alias::{Color, Km, ActionId, ActionsIdSet};
use crate::tsp::actions::action::Action;
use crate::tsp::actions::table_actions::TableActions;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;


//#[derive(Debug)]
//pub type TableActionIdToAction = HashMap<ActionId, Action>;

use std::fmt::Debug;
use crate::tsp::actions::db_controller::DBController;


#[derive(Debug)]
pub struct DatabaseActions {
    n: Color,
    b_max: Km,
    color_origin: Color,
    table: TableActions,

    controller: DBController
}

impl DatabaseActions {
    pub fn new(n: Color, b_max: Km, color_origin: Color) -> Self { 
        let mut instance = _new(n, b_max, color_origin);
        instance._init();
        return instance;
    }

    pub fn register_up(&mut self, km : Km, up_color: Color, parents : ActionsIdSet) -> ActionId {
        let action_up = Action::new_up(self.n, km, up_color, parents);
        return self._register_action(action_up);
    }

    pub fn get_action<'user>(&'user self, action_id : &'user ActionId) -> Option<&'user Action> {
        return self.table.get(action_id);
    }

    pub fn get_mut_action<'user>(&'user mut self, action_id : &'user ActionId) -> Option<&'user mut Action> {
        return self.table.get_mut(action_id);
    }

    pub fn remove(&mut self, action_id : &ActionId){
        self.table.delete(action_id);
    }

    pub fn can_use_it(&mut self, action_id : &ActionId) -> bool {
        return self.table.have(action_id) && self.controller.can_use_it(action_id);
    }

    pub fn use_it(&mut self, action_id : &ActionId){
        self.controller.use_it(action_id);
    }

    pub fn reserve(&mut self, action_id : &ActionId){
        self.controller.reserve(action_id);
    }

    pub fn reserve_destines(&mut self, destines : &Vec<ActionId>){
        for parent_id in destines.iter() {
            self.controller.reserve(parent_id);
        }
    }

    pub fn clean_db(&mut self){

        let pending_to_clean = self.controller.pending_to_clean_ids().clone();

        if !pending_to_clean.is_empty() {
            for action_id in  pending_to_clean.iter(){
                //println!("[DB Clean] Free action_id: {}",action_id);
                self.remove(action_id);
            }

            self.controller.clean_db_done();
        }
    }
}

impl DatabaseActions {   

    fn _init(&mut self){
        let action_init = Action::new_init(self.n, self.b_max,self.color_origin);
        self._register_action(action_init);
    }

    fn _register_action(&mut self, action : Action) -> ActionId {
        let action_id = action.id();
        self.table.put(action_id.clone(), action);

        return action_id;
    }

}

fn _new(n: Color, b_max: Km, color_origin: Color) -> DatabaseActions { 
    let table : TableActions = TableActions::new();
    let controller : DBController = DBController::new();
    DatabaseActions { n, b_max, color_origin, table, controller } 
}


impl fmt::Display for DatabaseActions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut txt : String = String::new();
        for (k, v ) in self.table.to_list(){
            txt = format!("{} id: {}, val: {}\n", txt, k, v);
        }

        return write!(f, "{}", txt);
    }
}

