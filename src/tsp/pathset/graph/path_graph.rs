use std::collections::HashMap;

use crate::tsp::utils::alias::{Color, Step, ActionId, ActionsIdSet};

pub type TableNodes = HashMap<ActionId, HashMap<NodeId, Node>>;
pub type TableEdges = HashMap<EdgeId, Edge>;
pub type TableLines = HashMap<Step, NodesIdSet>;
pub type TableColorNodes = HashMap<Color, NodesIdSet>;


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
    table_nodes : TableNodes,
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