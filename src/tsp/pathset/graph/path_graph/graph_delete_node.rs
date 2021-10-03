use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;


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

            self._delete_edges_parents(parents);
            self._delete_edges_sons(sons);
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

/*

# $ O(N) $
function delete_node!(graph :: Graph, node_id :: NodeId)
    if have_node(graph, node_id) && graph.valid
        node = get_node(graph, node_id)
        make_delete_node!(graph, node)
    end
end

# $ O(N) $
function make_delete_node!(graph :: Graph, node :: Node)
    if graph.valid
        delete_node_of_line!(graph, node)
        delete_node_of_table_colors!(graph, node)
        # $ O(N) $
        delete_edges_parents!(graph, node)
        # $ O(N) $
        delete_edges_sons!(graph, node)
        delete_node_of_table_nodes!(graph, node)
    end
end

function delete_node_of_line!(graph :: Graph, node :: Node)
    if haskey(graph.table_lines, node.step) && graph.valid
        delete!(graph.table_lines[node.step], node.id)
    end
end

function delete_node_of_table_colors!(graph :: Graph, node :: Node)
    if haskey(graph.table_color_nodes, node.color) && graph.valid
        nodes_color = graph.table_color_nodes[node.color]

        if node.id in nodes_color
            pop!(nodes_color, node.id)
        end
    end
end

# $ O(N) $
function delete_edges_parents!(graph :: Graph, node :: Node)
    if graph.valid
        destine_id = node.id

        # each node can have $ O(N-2) $  parents
        for (origin_id, edge_id) in node.parents
            delete_edge_by_id!(graph, edge_id)
        end
    end
end

# $ O(N) $
function delete_edges_sons!(graph :: Graph, node :: Node)
    if graph.valid
        origin_id = node.id
        # each node can have $ O(N-1) $ sons
        for (destine_id, edge_id) in node.sons
            delete_edge_by_id!(graph, edge_id)
        end
    end
end

function delete_node_of_table_nodes!(graph :: Graph, node :: Node)
    action_group_node = graph.table_nodes[node.action_id]
    delete!(action_group_node, node.id)

    if isempty(action_group_node)
        delete!(graph.table_nodes, node.action_id)
    end
end

*/