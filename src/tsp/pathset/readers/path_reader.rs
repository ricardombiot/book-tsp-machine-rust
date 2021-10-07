use crate::tsp::utils::alias::{Step, Color, Km};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::graph::path_graph::PathGraph;

pub struct PathSolutionReader {
    step : Step,
    route : Vec<Color>,
    next_node_id : Option<NodeId>,
    owners : OwnersByStep,
    
    graph : PathGraph,
    is_origin_join : bool
}

impl PathSolutionReader {
    pub fn read(n : Color, b_max : Km, graph : &PathGraph) ->  PathSolutionReader {
        let mut path = PathSolutionReader::new(n, b_max, graph);
        path.calc();
        return path;
    }
}


pub mod path_reader_constructor;
pub mod path_reader_next_step;
pub mod path_reader_selection;
pub mod path_reader_reduce;