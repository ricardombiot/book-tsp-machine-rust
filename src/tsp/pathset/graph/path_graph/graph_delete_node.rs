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

    fn _save_to_delete(&mut self, node_id : NodeId) {
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
            let nodes_to_delete =  &self.nodes_to_delete;
            let table_nodes_by_action =  &mut self.table_nodes_by_action;

            for node_id in nodes_to_delete {
                table_nodes_by_action._delete_node(node_id);
            }
        }
        /*
        if graph.valid
            if !isempty(graph.nodes_to_delete)
                node_id = pop!(graph.nodes_to_delete)
    
                # $ O(N) $
                delete_node!(graph, node_id)
    
                graph.required_review_ownwers = true
                apply_node_deletes!(graph)
            end
        end*/
    }

}
/*

# $ O(N^4) $
function delete_node_by_color!(graph :: Graph, color :: Color)
    if graph.valid
        # in each machine km, only can produce a action by color (origin node),
        # each action can produce $ O(N) $ nodes by color then
        # $ O(N^2) $ of each color
        for node_id in get_nodes_by_color(graph, color)
            save_to_delete_node!(graph, node_id)

            if !graph.valid
                break
            end
        end

        # $ O(N^4) $ deleting all nodes
        apply_node_deletes!(graph)
    end
end

function save_to_delete_node!(graph :: Graph, node_id :: NodeId)
    if graph.valid
        node = get_node(graph, node_id)

        if node != nothing
            save_to_delete_node!(graph, node)
        end
    end
end
function save_to_delete_node!(graph :: Graph, node :: Node)
    pop_owner_in_graph!(graph, node)
    push!(graph.nodes_to_delete, node.id)
end

function pop_owner_in_graph!(graph :: Graph, node_owner :: Node)
    if graph.valid
        Owners.pop!(graph.owners, node_owner.step, node_owner.id)
        graph.required_review_ownwers = true
        make_validation_graph_by_owners!(graph)
    end
end

function make_validation_graph_by_owners!(graph :: Graph)
    graph.valid = graph.owners.valid
end

# Theoretical Maximum: $ O(N^3) nodes * O(N) = O(N^4) $
function apply_node_deletes!(graph :: Graph)
    # It will be execute less than $ O(N^3) $ delete total nodes
    # before will detected that graph is unvalid and avoid
    # the execution of delete nodes.
    if graph.valid
        if !isempty(graph.nodes_to_delete)
            node_id = pop!(graph.nodes_to_delete)

            # $ O(N) $
            delete_node!(graph, node_id)

            graph.required_review_ownwers = true
            apply_node_deletes!(graph)
        end
    end
end

*/