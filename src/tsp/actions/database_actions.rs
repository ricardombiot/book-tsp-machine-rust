use std::fmt;
use std::collections::HashMap;
use crate::tsp::utils::alias::{Color, Km, ActionId, ActionsIdSet};
use crate::tsp::actions::action::Action;

pub type TableActionIdToAction = HashMap<ActionId, Action>;

pub struct DatabaseActions {
    n: Color,
    b_max: Km,
    color_origin: Color,
    table: TableActionIdToAction
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

    pub fn get_action(&self, action_id : &ActionId) -> Option<&Action> {
        return self.table.get(action_id);
    }

    pub fn get_mut_action(&mut self, action_id : &ActionId) -> Option<&mut Action> {
        return self.table.get_mut(action_id);
    }

    pub fn remove(&mut self, action_id : ActionId){
        self.table.remove(&action_id);
    }
}

impl DatabaseActions {   

    fn _init(&mut self){
        let action_init = Action::new_init(self.n, self.b_max,self.color_origin);
        self._register_action(action_init);
    }

    fn _register_action(&mut self, action : Action) -> ActionId {
        let action_id = action.id();
        self.table.insert(action_id.clone(), action);

        return action_id;
    }

}


impl fmt::Display for DatabaseActions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut txt : String = String::new();
        for (k, v ) in self.table.iter(){
            txt = format!("{} id: {}, val: {}\n", txt, k, v);
        }

        return write!(f, "{}", txt);
    }
}

fn _new(n: Color, b_max: Km, color_origin: Color) -> DatabaseActions { 
    let table : TableActionIdToAction = TableActionIdToAction::new();
    DatabaseActions { n, b_max, color_origin, table } 
}