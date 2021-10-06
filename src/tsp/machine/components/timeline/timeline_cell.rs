use crate::tsp::utils::alias::{Km,Color, ActionsIdSet, ActionId};
use crate::tsp::actions::action::Action;
use crate::tsp::actions::database_actions::DatabaseActions;

#[derive(Clone, Debug)]
pub struct TimelineCell {
    km : Km,
    color : Color,
    parents : ActionsIdSet ,
    action_id : Option<ActionId>,
    valid : bool
}

impl TimelineCell {
    pub fn new(km: Km, color: Color, action_id: Option<ActionId>) -> Self { 
        let valid: bool = true;
        let parents: ActionsIdSet = ActionsIdSet::new();
        Self { km, color, parents, action_id, valid } 
    }

    pub fn push_parent(&mut self, parent_id : ActionId){
        self.parents.insert(parent_id);
    }

    pub fn get_action<'user>(&'user self, db: &'user DatabaseActions) -> Option<&'user Action>{
        match self.action_id {
            None => None,
            Some(action_id) => db.get_action(&action_id)
        }
    }

    pub fn was_execute(&self) -> bool {
        return self.action_id.is_some();
    }

    pub fn execute(&mut self, db: & mut DatabaseActions){
        let is_pending = !self.was_execute();
        if is_pending {

            let up_action_id = db.register_up(self.km, self.color, self.parents.clone());

            // ... MORE ... execute action run!

            self.action_id = Some(up_action_id);
        }
    }
}