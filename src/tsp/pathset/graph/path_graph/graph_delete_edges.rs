use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {

    pub(super) fn _delete_edges_sons(&mut self, sons: Vec<(NodeId,EdgeId)>) {
        let table_edges = &mut self.table_edges;
        let table_nodes_by_action = &mut self.table_nodes_by_action;

        for (node_id, edge_id) in sons.iter() {
            table_edges.delete(edge_id);
            let action_id = node_id.action_id();

            let node = table_nodes_by_action.get_node_mut(&action_id, node_id);
            match node {
                None => {},
                Some(node) => node.delete_parent(node_id)
            }
        }
    }

    pub(super) fn _delete_edges_parents(&mut self, parents: Vec<(NodeId,EdgeId)>) {
        let table_edges = &mut self.table_edges;
        let table_nodes_by_action = &mut self.table_nodes_by_action;

        for (node_id, edge_id) in parents.iter() {
            table_edges.delete(edge_id);
            let action_id = node_id.action_id();

            let node = table_nodes_by_action.get_node_mut(&action_id, node_id);
            match node {
                None => {},
                Some(node) => node.delete_son(node_id)
            }
        }
    }


}
