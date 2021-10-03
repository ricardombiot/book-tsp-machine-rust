
use crate::tsp::utils::alias::{Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {
    pub fn get_lenght(&self) -> Step {
        self.next_step
    }

    pub fn get_id_origin(&self) -> Option<NodeId> {
        let step_init = 0 as Step;
        let set_nodes = self.table_lines.get(&step_init).unwrap();
        match set_nodes.iter().next() {
            None => None,
            Some(node_id) => Some(node_id.clone())
        }
    }

    pub fn get_action_id_origin(&self) -> Option<ActionId> {
        match self.get_id_origin() {
            None => None,
            Some(node_id) => Some(node_id.action_id())
        }
    }

}

