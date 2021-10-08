use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::graph::path_graph::PathGraph;


use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
use crate::tsp::pathset::graph::path_graph::table_edges::TableEdges;
use crate::tsp::pathset::graph::path_graph::table_lines::TableLines;
use crate::tsp::pathset::graph::path_graph::table_color_nodes::TableColorNodes;
use crate::tsp::pathset::graph::path_graph::table_nodes_by_action::TableNodesByAction;




impl PathGraph {

    pub fn new(n : Color, b_max : Km, color_origin : Color, action_id : ActionId) -> Self {
        let next_step = 0 as Step;
        let owners_graph = PathGraph::_new_graph_owners(n, b_max);

        let table_nodes_by_action = TableNodesByAction::new();
        let table_color_nodes = TableColorNodes::new();
        let table_lines = TableLines::new();
        //let table_edges = TableEdges::new();

        let nodes_to_delete = NodesIdSet::new();

        let valid = true;
        let required_review_ownwers = false;
        let action_parent_id : Option<ActionId> = None;
        let max_review_stages = 0;

        let mut graph = PathGraph{n, b_max, next_step, color_origin, owners_graph,
            table_nodes_by_action, 
            table_lines, table_color_nodes,
            action_parent_id, nodes_to_delete, 
            required_review_ownwers, max_review_stages, valid};

        graph._init(action_id);
        return graph;
    }

    fn _new_graph_owners(n : Color, b_max : Km) -> OwnersByStep {
        return OwnersByStep::seed(n, b_max)
    }

}

/*
function new(n :: Color, b :: Km, color_origin :: Color, action_id :: ActionId)
    next_step = Step(0)
    owners = new_graph_owners(n, b)

    table_nodes = Dict{ActionId, Dict{NodeId, Node}}()
    table_color_nodes = Dict{Color, NodesIdSet}()
    table_lines = Dict{Step, NodesIdSet}()
    table_edges = Dict{EdgeId, Edge}()
    nodes_to_delete = NodesIdSet()

    required_review_ownwers = false
    valid = true
    action_parent_id = nothing
    max_review_stages = 0

    graph = Graph(n, b, next_step, color_origin, owners,
            table_nodes, table_edges,
            table_lines, table_color_nodes,
            action_parent_id,
            nodes_to_delete, required_review_ownwers, max_review_stages, valid)

    init!(graph, action_id)
    return graph
end

function new_graph_owners(n :: Color, b :: Km) :: OwnersByStep
    # Para poder hacer un join con origin
    # tenemos que añadir un km más
    b = b + 1
    n = n + 1

    #bbnnn = Int64(b^2*n^3)
    bbnnn :: UniqueNodeKey = b^2*n^3
    return Owners.new(bbnnn)
end
*/