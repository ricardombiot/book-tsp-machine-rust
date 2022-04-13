use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
use crate::tsp::utils::alias::{Step, Color, Km};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;

impl PathSolutionReader {

    pub fn new(n : Color, b_max : Km, graph : &PathGraph) -> Self {
        let is_origin_join : bool = false;
        let step = 0 as Step;
        let route : Vec<Color> = Vec::new();

        let next_node_id = PathSolutionReader::_get_fisrt_node_id(n,b_max, graph);
        let owners = graph.owners_graph().empty_derive();
        let graph = graph.clone();

        Self{step, route, next_node_id, owners, graph, is_origin_join}
    }

    fn _get_fisrt_node_id(n : Color, b_max : Km, graph : &PathGraph) -> Option<NodeId> {
        let action_id_origin = graph.get_action_id_origin().unwrap();
        return Some(NodeId::new_root(n, b_max, action_id_origin));
    }
}
