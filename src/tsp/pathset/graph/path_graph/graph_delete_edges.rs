use crate::tsp::pathset::components::nodes::node_id::{NodeId, NodesIdSet};
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {


    pub(super) fn _delete_edges_sons_and_save_nodes_with_empty_parents(&mut self, sons: Vec<(NodeId,EdgeId)>, node_id : &NodeId) {
        let sons_to_remove = self._delete_edges_sons(sons, node_id);
        self.save_to_delete_using_set(sons_to_remove);
    } 

    pub(super) fn _delete_edges_sons(&mut self, sons: Vec<(NodeId,EdgeId)>, node_id : &NodeId) -> NodesIdSet {
        let table_edges = &mut self.table_edges;
        let table_nodes_by_action = &mut self.table_nodes_by_action;
        let mut nodes_to_delete : NodesIdSet = NodesIdSet::new();

        for (son_node_id, edge_id) in sons.iter() {
            table_edges.delete(edge_id);
            let action_id = son_node_id.action_id();

            let son_node = table_nodes_by_action.get_node_mut(&action_id, son_node_id);
            if son_node.is_some() {
                let son_node = son_node.unwrap();

                son_node.delete_parent(node_id);

                if !son_node.have_parents() {
                    nodes_to_delete.insert(node_id.clone());
                }
            }
        }

        return nodes_to_delete;
    }

    pub(super) fn _delete_edges_parents_and_save_nodes_with_empty_sons(&mut self, parents: Vec<(NodeId,EdgeId)>, node_id : &NodeId) {
        let parents_to_remove = self._delete_edges_parents(parents, node_id);
        self.save_to_delete_using_set(parents_to_remove);
    } 

    pub(super) fn _delete_edges_parents(&mut self, parents: Vec<(NodeId,EdgeId)>, node_id : &NodeId) -> NodesIdSet {
        let table_edges = &mut self.table_edges;
        let table_nodes_by_action = &mut self.table_nodes_by_action;
        let mut nodes_to_delete : NodesIdSet = NodesIdSet::new();

        for (parent_node_id, edge_id) in parents.iter() {
            table_edges.delete(edge_id);
            let action_id = node_id.action_id();

            let parent_node = table_nodes_by_action.get_node_mut(&action_id, parent_node_id);
            if parent_node.is_some() {
                let parent_node = parent_node.unwrap();

                parent_node.delete_son(node_id);

                if !parent_node.have_sons() {
                    nodes_to_delete.insert(node_id.clone());
                }
            }
        }

        return nodes_to_delete;
    }


}
