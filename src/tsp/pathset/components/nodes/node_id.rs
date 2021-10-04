use crate::tsp::utils::alias::{Color, Km, Step, ActionId, UniqueNodeKey, InfoActionId};
use crate::tsp::utils::generator_node_key;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Eq, Clone)]
pub struct NodeId {
    key : UniqueNodeKey,
    step : Step,
    action_id : ActionId,
    action_parent_id : ActionId
}

pub type NodesIdSet = HashSet<NodeId> ;

impl NodeId {
    pub fn new(n : Color, b_max : Km, step : Step, action_id : ActionId, action_parent_id : ActionId) -> Self{
        return _new(n, b_max, step, action_id, Some(action_parent_id));
    }

    pub fn new_root(n : Color, b_max : Km, action_id : ActionId) -> Self {
        let step: Step = 0 as Step;
        return _new(n, b_max, step, action_id, None);
    }

    pub fn is_root(&self) -> bool {
        return self.action_id == self.action_parent_id;
    }

    pub fn get_info_node_id(&self, n : Color) -> (InfoActionId, InfoActionId) {
        return generator_node_key::get_info_by_actions(n, self.action_id, self.action_parent_id);
    }

    pub fn key(&self) -> UniqueNodeKey {
        self.key
    }

    pub fn step(&self) -> Step {
        self.step
    }

    pub fn action_id(&self) -> ActionId {
        self.action_id
    }

    pub fn action_parent_id(&self) -> ActionId {
        self.action_parent_id
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "K{}", self.key);
    }
}

impl PartialEq for NodeId {
    fn eq(&self, node_id_b: &Self) -> bool {
        self.key == node_id_b.key()
    }
}
use std::hash::{Hash, Hasher};

impl Hash for NodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

fn _new(n : Color, b_max : Km, step : Step, action_id : ActionId, action_parent_id : Option<ActionId>) -> NodeId {
    let my_action_parent_id = match action_parent_id {
        Some(parent_id) => parent_id,
        None => action_id
    };

    let key = generator_node_key::calc_unique_node_key(n, b_max, step, action_id, my_action_parent_id);
    return NodeId{key: key, step: step , action_id: action_id, action_parent_id: my_action_parent_id};
}