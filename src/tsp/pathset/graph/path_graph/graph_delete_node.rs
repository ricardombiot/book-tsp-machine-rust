use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::alias::{Color, Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;



impl PathGraph {
    pub(super) fn _delete_node(&mut self, node_id : &NodeId){
        let exists = self._have_node(node_id);
        if self.valid && exists {
            let action_id : ActionId = node_id.action_id();
            let node = self.table_nodes_by_action.get_node(&action_id, &node_id).unwrap();
            let color : &Color = &node.color();
            let step : &Step = &node.step();
            let parents = node.parents_list();
            let sons = node.sons_list();

            self.table_color_nodes.delete_node(color, node_id);
            self.table_lines.delete_node(step, node_id);

            //let parents_to_remove = self._delete_edges_parents(parents, node_id);
            //self.save_to_delete_using_set(parents_to_remove);
            self._delete_edges_parents_and_save_nodes_with_empty_sons(parents, node_id);
            self._delete_edges_sons_and_save_nodes_with_empty_parents(sons, node_id);
            //let sons_to_remove = self._delete_edges_sons
            //self.save_to_delete_using_set(sons_to_remove);

            self._table_nodes_delete_node(node_id);
        }
    }

    

    pub(super) fn _have_node(&self, node_id : &NodeId) -> bool {
        self.table_nodes_by_action.have_node(node_id)
    }

    fn _table_nodes_delete_node(&mut self, node_id : &NodeId) {
        self.table_nodes_by_action.delete_node(node_id);
    }

}
