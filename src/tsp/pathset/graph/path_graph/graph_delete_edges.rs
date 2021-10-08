use crate::tsp::pathset::components::edges::edge;
use crate::tsp::pathset::components::nodes::node_id::{NodeId, NodesIdSet};
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {


    pub(super) fn _delete_edges_sons_and_save_nodes_with_empty_parents(&mut self, sons: Vec<NodeId>, node_id : &NodeId) {
        let sons_to_remove = self._delete_edges_sons(sons, node_id);
        self.save_nodes_to_delete(sons_to_remove);
    } 

    pub(super) fn _delete_edges_sons(&mut self, sons: Vec<NodeId>, node_id : &NodeId) -> Vec<NodeId> {
        let mut nodes_to_delete : Vec<NodeId> = Vec::new();
        let origin_id = node_id;
        for destine_id in sons.iter() {
            let list_to_delete = self._delete_edge(origin_id, destine_id);
            nodes_to_delete.extend(list_to_delete);
        }

        return nodes_to_delete;
    }

    /*pub(super) fn _delete_edges_sons(&mut self, sons: Vec<(NodeId,EdgeId)>, node_id : &NodeId) -> NodesIdSet {
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
    }*/

    fn _delete_edge(&mut self, origin_id : &NodeId, destine_id : &NodeId) -> Vec<NodeId> {
        //let origin_id = edge_id.origin_id();
        let origin_action_id = &origin_id.action_id();
        //let destine_id = edge_id.destine_id();
        let destine_action_id = &destine_id.action_id();
        let mut list_nodes_to_remove : Vec<NodeId> = Vec::new();


        match self.table_nodes_by_action.get_node_mut(origin_action_id,origin_id) {
            Some(origin_node) => {
                origin_node.delete_son(destine_id);

                if !origin_node.have_sons() {
                    list_nodes_to_remove.push(origin_id.clone())
                }
            },
            _ => {}
        }

        match self.table_nodes_by_action.get_node_mut(destine_action_id,destine_id) {
            Some(destine_node) => {
                destine_node.delete_parent(origin_id);

                if !destine_node.have_parents() {
                    list_nodes_to_remove.push(destine_id.clone())
                }
            },
            _ => {}
        }
       
        //self.table_edges.delete(edge_id);
        return list_nodes_to_remove;
    }

    pub(super) fn _delete_edges_parents_and_save_nodes_with_empty_sons(&mut self, parents: Vec<NodeId>, node_id : &NodeId) {
        let parents_to_remove = self._delete_edges_parents(parents, node_id);
        self.save_nodes_to_delete(parents_to_remove);
    } 


    pub(super) fn _delete_edges_parents(&mut self, sons: Vec<NodeId>, node_id : &NodeId) -> Vec<NodeId> {
        let mut nodes_to_delete : Vec<NodeId> = Vec::new();
        let destine_id = node_id;
        for origin_id in sons.iter() {
            let list_to_delete = self._delete_edge(origin_id, destine_id);
            nodes_to_delete.extend(list_to_delete);
        }

        return nodes_to_delete;
    }

    /*
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
     */


}
