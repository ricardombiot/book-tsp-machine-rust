use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::pathset::components::nodes::node_id::NodeId;

use super::table_nodes_by_action;

impl PathGraph {
    pub(super) fn _delete_node_by_color(&mut self, color : Color){
        let exist_color = self.table_color_nodes.have(&color);
        if self.valid && exist_color {
            /* in each machine km, only can produce a action by color (origin node),
             each action can produce $ O(N) $ nodes by color then
             $ O(N^2) $ of each color*/
            let set_nodes_by_color = self.table_color_nodes.get(&color).unwrap().clone();
            for node_id in set_nodes_by_color.iter() {
                self._save_to_delete(node_id.clone());

                if !self.valid {
                    break;
                }
            }

            //# $ O(N^4) $ deleting all nodes
            self._apply_node_deletes()
        }
    }

    pub(super) fn _save_to_delete(&mut self, node_id : NodeId) {
        self.nodes_to_delete.insert(node_id.clone());
        self.owners_graph.pop(&node_id);
        self.required_review_ownwers = true;
        self.valid = self.owners_graph.valid();
    }

    pub(super) fn _apply_node_deletes(&mut self){
        /*
        # It will be execute less than $ O(N^3) $ delete total nodes
        # before will detected that graph is unvalid and avoid
        # the execution of delete nodes.
        */
        let have_nodes_to_delete = !self.nodes_to_delete.is_empty();
        if self.valid && have_nodes_to_delete {
            let nodes_to_delete =  &self.nodes_to_delete.clone();
            //let table_nodes_by_action =  &mut self.table_nodes_by_action;

            for node_id in nodes_to_delete {
                //table_nodes_by_action._delete_node(node_id);
                self._delete_node(node_id);
                self.required_review_ownwers = true;

                if !self.valid {
                    break;
                }
            }
        }
    }

}