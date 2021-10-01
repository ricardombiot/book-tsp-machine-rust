use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::components::owners::owners::OwnersByStep;

use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
use crate::tsp::pathset::graph::path_graph::table_edges::TableEdges;
use crate::tsp::pathset::graph::path_graph::table_lines::TableLines;
use crate::tsp::pathset::graph::path_graph::table_color_nodes::TableColorNodes;
use crate::tsp::pathset::graph::path_graph::table_nodes_by_action::TableNodesByAction;

pub mod table_edges;
pub mod table_lines;
pub mod table_color_nodes;
pub mod table_nodes;
pub mod table_nodes_by_action;

pub struct PathGraph {
    // N of nodes
    n : Color,
    // max km target (B)
    b : Km,
    // Next step (length of step)
    next_step : Step,
    // Color origin (node origin)
    color_origin : Color,
    // Owners of graph
    owners : OwnersByStep,

    // dictionary of nodes by actionid and node id
    table_nodes : TableNodesByAction,
    // dictionary of edges by edge id
    table_edges : TableEdges,
    // dictionary of node id by line
    table_lines : TableLines,
    // dictionary of node id by color
    table_color_nodes : TableColorNodes,

    // save the parent action id
    action_parent_id : Option<ActionId>,
    // temporal set of nodes id that should be delete
    nodes_to_delete : NodesIdSet,
    // flag that say if is required make a review of owners
    required_review_ownwers : bool,
    // info max stage review
    max_review_stages : u32,
    // flag that say if the graph is valid
    valid : bool
}

impl PathGraph {
    
}