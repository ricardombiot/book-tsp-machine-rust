use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::components::owners::owners::OwnersByStep;

use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
//use crate::tsp::pathset::graph::path_graph::table_edges::TableEdges;
use crate::tsp::pathset::graph::path_graph::table_lines::TableLines;
use crate::tsp::pathset::graph::path_graph::table_color_nodes::TableColorNodes;
use crate::tsp::pathset::graph::path_graph::table_nodes_by_action::TableNodesByAction;
use std::fmt::Debug;

pub mod table_edges;
pub mod table_lines;
pub mod table_color_nodes;
pub mod table_nodes;
pub mod table_nodes_by_action;

#[derive(Clone, Debug)]
pub struct PathGraph {
    // N of nodes
    n : Color,
    // max km target (B)
    b_max : Km,
    // Next step (length of step)
    next_step : Step,
    // Color origin (node origin)
    color_origin : Color,
    // Owners of graph
    owners_graph : OwnersByStep,

    // dictionary of nodes by actionid and node id
    table_nodes_by_action : TableNodesByAction,
    // dictionary of edges by edge id
    //table_edges : TableEdges,
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

pub mod graph_constructor;
pub mod graph_init;
pub mod graph_add_node;
pub mod graph_delete;
pub mod graph_delete_node;
pub mod graph_delete_edges;
pub mod graph_getters;
pub mod graph_join;
pub mod graph_make_up;
pub mod graph_owners_review;
pub mod graph_owners_filters;
pub mod graph_owners_colors_review;

pub mod graph_to_dot;

impl PathGraph {
    pub fn n(&self) -> Color {
        self.n
    }

    pub fn b_max(&self) -> Km {
        self.b_max
    }


    pub fn next_step(&self) -> Step {
        self.next_step
    }

    pub fn color_origin(&self) -> Color {
        self.color_origin
    }

    pub fn owners_graph(&self) -> &OwnersByStep {
        &self.owners_graph
    }

    pub fn table_nodes_by_action(&self) -> &TableNodesByAction {
        &self.table_nodes_by_action
    }

    /* 
    pub fn table_edges(&self) -> &TableEdges {
        &self.table_edges
    }*/

    pub fn table_lines(&self) -> &TableLines {
        &self.table_lines
    }

    pub fn table_color_nodes(&self) -> &TableColorNodes {
        &self.table_color_nodes
    }

    pub fn action_parent_id(&self) -> &Option<ActionId> {
        &self.action_parent_id
    }

    pub fn nodes_to_delete(&self) -> &NodesIdSet {
        &self.nodes_to_delete
    }

    pub fn required_review_ownwers(&self) -> bool {
        self.required_review_ownwers
    }

    pub fn max_review_stages(&self) -> u32 {
        self.max_review_stages
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
}