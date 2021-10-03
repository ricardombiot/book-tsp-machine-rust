use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::components::nodes::node::dict_edgeid_by_nodeid::DictEdgeIdByNodeId;

pub mod dict_edgeid_by_nodeid;
pub mod node_parents_and_sons;
pub mod node_owners;
pub mod node_join;

#[derive(Clone)]
pub struct Node {
    id : NodeId,
    action_id : ActionId,
    color : Color,
    step : Step,

    parents : DictEdgeIdByNodeId,
    sons : DictEdgeIdByNodeId,

    owners : OwnersByStep
}


impl Node {
    pub fn new_root(n : Color, b_max : Km, color : Color, owners_graph : &OwnersByStep, action_id : ActionId) -> Self {
        Node::create(n, b_max, 0 as Step, color, owners_graph, action_id, None)
    }
    pub fn new(n : Color, b_max : Km, step : Step, color : Color, owners_graph : &OwnersByStep, action_id : ActionId, action_parent_id : ActionId) -> Self {
        Node::create(n, b_max, step, color, owners_graph, action_id, Some(action_parent_id))
    }

    pub fn create(n : Color, b_max : Km, step : Step, color : Color, owners_graph : &OwnersByStep, action_id : ActionId, action_parent_id : Option<ActionId>) -> Self {
        let parents  = DictEdgeIdByNodeId::new();
        let sons = DictEdgeIdByNodeId::new();
        let owners : OwnersByStep = owners_graph.derive();
        
        match action_parent_id {
            None => {
                let id = NodeId::new_root(n, b_max, action_id);
                let step = id.step();
                return Self {id, action_id, color, step, parents, sons, owners}
            }
            Some(action_parent_id) => {
                let id = NodeId::new(n, b_max, step, action_id, action_parent_id);
                return Self {id, action_id, color, step, parents, sons, owners}
            }
        }
    }

    pub fn is_root(&self) -> bool {
        self.id.is_root()
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn owners(&self) -> &OwnersByStep {
        &self.owners
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn step(&self) -> Step {
        self.step.clone()
    }

    pub fn action_id(&self) -> ActionId {
        self.action_id.clone()
    }


}

