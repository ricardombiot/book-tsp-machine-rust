use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;


impl PathGraph {
    pub(super) fn _add_node(&mut self,node : Node) {
        let action_id = node.action_id();
        let node_id = node.id().clone();

        self.table_nodes_by_action.add_node(node);
        let node: &mut Node = self.table_nodes_by_action.get_node_mut(&action_id, &node_id).unwrap();
        self.table_color_nodes.add_node_color(node);
        

/* 
        let node2: &mut Node = self.table_nodes_by_action.get_node_mut(&action_id, &node_id).unwrap();

        self.table_color_nodes.add_node_color(node);*/
    }
}

/*
# Theoretical Maximum: $ O(N^3) $
# Init case: $ O(1) $
function add_node!(graph :: Graph, node :: Node)
    if !haskey(graph.table_nodes, node.action_id)
        graph.table_nodes[node.action_id] = Dict{NodeId, Node}()
    end

    graph.table_nodes[node.action_id][node.id] = node
    add_node_color!(graph, node)
    add_node_in_line!(graph, node)

    push_owner_myself_as_owner_of_me!(node)

    # $ O(N^3) $
    push_node_as_new_owner!(graph, node)
    push_owner_in_graph!(graph, node)
end

function add_node_color!(graph :: Graph, node :: Node)
    if !haskey(graph.table_color_nodes, node.color)
        graph.table_color_nodes[node.color] = NodesIdSet()
    end

    push!(graph.table_color_nodes[node.color], node.id)
end

function add_node_in_line!(graph :: Graph, node :: Node)
    if haskey(graph.table_lines, node.step)
        push!(graph.table_lines[node.step], node.id)
    end
end

function push_owner_myself_as_owner_of_me!(node :: Node)
    PathNode.push_owner!(node, node)
end

# $ O(Steps) * O(N^2) = O(N^3) $ maximum number of nodes in graph
function push_node_as_new_owner!(graph :: Graph, node_owner :: Node)
    # The cost will be proportional to length of graph then
    # graph.step * $ O(N^2) $ by step, until $ O(N^3) $

    # $ O(N) $ steps * $ O(N) $ actions by step
    for (action_id, table_nodes_action) in graph.table_nodes
        # $ O(N) $ nodes by action
        for (node_id, node) in table_nodes_action
            PathNode.push_owner!(node, node_owner)
        end
    end
end

function push_owner_in_graph!(graph :: Graph, node_owner :: Node)
    Owners.push!(graph.owners, node_owner.step, node_owner.id)
end
*/