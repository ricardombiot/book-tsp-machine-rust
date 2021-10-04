use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::utils::alias::{Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::graph::path_graph::OwnersByStep;
use crate::tsp::pathset::components::nodes::node::Node;

impl PathGraph {

    // # $ O(N^3) $
    pub(super) fn _filter_by_intersection_owners(&mut self, node_id : &NodeId) -> bool {
        let action_id : ActionId = node_id.action_id();
        let node = self.table_nodes_by_action.get_node_mut(&action_id, node_id).unwrap();
        node.intersect_owners(&self.owners_graph);

        let should_be_filter = !node.is_valid();
        return should_be_filter;
    }

    pub(super) fn _filter_by_parents_intersection_owners(&mut self, node_id : &NodeId) -> bool {
        let mut should_be_filter = true;
        let action_id : ActionId = node_id.action_id();
        let node = self.table_nodes_by_action.get_node(&action_id, node_id).unwrap();
        
        let first_step = 0 as Step;
        let is_node_in_first_step = node.step() == first_step;
        if is_node_in_first_step {
            should_be_filter = false;
        }else{
            let parents_list = node.parents_list();
            let owners_parents_union : Option<OwnersByStep> = self._calc_union_owners(parents_list);
    
            if owners_parents_union.is_none() {
                should_be_filter = true;
            }else{
                let owners_union = owners_parents_union.unwrap();
                let is_owners_union_not_valid = !owners_union.valid();
                if is_owners_union_not_valid {
                    should_be_filter = true;
                }else{
                    let node = self.table_nodes_by_action.get_node_mut(&action_id, node_id).unwrap();

                    node.intersect_owners(&owners_union);
                    let is_node_invalid = !node.is_valid();
                    if is_node_invalid {
                        should_be_filter = true;
                    }else{
                        self._remove_son_edges_arent_owner_node(node_id);
                        should_be_filter = false;
                    }
                }
            }
        } 

        return should_be_filter;
    }


    pub(super) fn _filter_by_sons_intersection_owners(&mut self, node_id : &NodeId) -> bool {
        let mut should_be_filter = true;
        let action_id = node_id.action_id();
        let node = self.table_nodes_by_action.get_node_mut(&action_id, node_id).unwrap();
        
        let last_step = self.next_step - (1 as Step);
        let is_node_in_last_step = node.step() == last_step;
        if is_node_in_last_step {
            should_be_filter = false;
        }else{
            let sons_list = node.sons_list();
            let owners_sons_union : Option<OwnersByStep> = self._calc_union_owners(sons_list);
    
            if owners_sons_union.is_none() {
                should_be_filter = true;
            }else{
                let owners_union = owners_sons_union.unwrap();
                let is_owners_union_not_valid = !owners_union.valid();
                if is_owners_union_not_valid {
                    should_be_filter = true;
                }else{
                    let node = self.table_nodes_by_action.get_node_mut(&action_id, node_id).unwrap();
                    
                    node.intersect_owners(&owners_union);
                    let is_node_invalid = !node.is_valid();
                    if is_node_invalid {
                        should_be_filter = true;
                    }else{
                        self._remove_parents_edges_arent_owner_node(node_id);
                        should_be_filter = false;
                    }
                }
            }
        } 

        return should_be_filter;
    }


    fn _calc_union_owners(&self, parents_list : Vec<(NodeId,EdgeId)>) -> Option<OwnersByStep> {
        let mut owners_parents_union : Option<OwnersByStep> = None;
        for (parent_node_id, _edge_id) in parents_list.iter() {
            let action_parent_id = parent_node_id.action_id();
            let node_parent = self.table_nodes_by_action.get_node(&action_parent_id, parent_node_id).unwrap();

            if node_parent.is_valid() {
                match owners_parents_union.as_mut() {
                    None => { 
                        owners_parents_union = Some(node_parent.owners().derive()) 
                    }
                    Some(owners_union) => {
                        owners_union.union(node_parent.owners());
                    }
                }
            }
        }

        return owners_parents_union;
    }

    fn _remove_son_edges_arent_owner_node(&mut self, node_id : &NodeId){
        let action_id = node_id.action_id();
        let node = self.table_nodes_by_action.get_node(&action_id, node_id).unwrap();
        let sons_list = node.sons_list();
        let sons_list_to_remove = self._calc_edges_arent_owners_node(node,sons_list);

        if !sons_list_to_remove.is_empty(){
            self._delete_edges_sons(sons_list_to_remove)
        }
    }

    fn _remove_parents_edges_arent_owner_node(&mut self, node_id : &NodeId){
        let action_id = node_id.action_id();
        let node = self.table_nodes_by_action.get_node(&action_id, node_id).unwrap();

        let parents_list = node.parents_list();
        let parents_list_to_remove = self._calc_edges_arent_owners_node(node,parents_list);

        if !parents_list_to_remove.is_empty(){
            self._delete_edges_parents(parents_list_to_remove)
        }
    }

    fn _calc_edges_arent_owners_node(&self, node : &Node, list_parents_or_sons : Vec<(NodeId,EdgeId)>) -> Vec<(NodeId,EdgeId)>{
        let mut list_to_remove : Vec<(NodeId,EdgeId)> = Vec::new();

        for tuple_parent in list_parents_or_sons.iter() {
            let parent_node_id = &tuple_parent.0;

            let should_be_remove_edge = !node.have_owner(parent_node_id);
            if should_be_remove_edge {
                list_to_remove.push(tuple_parent.clone());
            }

        }

        return list_to_remove;
    }



    /*
    # $ O(N^4) $
function filter_by_parents_intersection_owners!(graph :: Graph, node :: Node) :: Bool
    first_step = Step(0)
    if node.step != first_step
        owners_parents_union :: Union{OwnersByStep,Nothing} = nothing

        # $ O(N) $
        for (parent_node_id, edge_id) in node.parents
            parent_node = get_node(graph, parent_node_id)

            if parent_node.owners.valid
                if owners_parents_union == nothing
                    owners_parents_union = deepcopy(parent_node.owners)
                else
                    # $ O(N) Steps * O(N^2) = O(N^3) $
                    Owners.union!(owners_parents_union, parent_node.owners)
                end
            end
        end

        if owners_parents_union == nothing
            return true
        elseif owners_parents_union.valid
            # $ O(N^3) $
            PathNode.intersect_owners!(node, owners_parents_union)
            if !node.owners.valid
                return true
            else
                # $ O(N) $
                if node.step != Step(0)
                    remove_sons_edges_arent_owner_node!(graph, node)
                end

                return false
            end
        else
            return false
        end
    else
        return false
    end
end
    */
}