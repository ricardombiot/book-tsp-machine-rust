
use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
use crate::tsp::utils::alias::{Step, Color, Km};
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathSolutionReader {

    pub(super) fn _fixed_next(&mut self){
        let next_node_id = self.next_node_id.clone().unwrap();
        let next_action_id = next_node_id.action_id();

        let node = self.graph.table_nodes_by_action().get_node(&next_action_id, &next_node_id).unwrap();

        self.next_node_id = self._select_next(node);

        println!("[{}] Selected: {:?}", self.step, self.next_node_id);
    }

    fn _select_next(&self, node : &Node) -> Option<NodeId> {
        if self._have_next(node) {
            let (node_id, _edge_id)  = node.take_one_son().unwrap();
            return Some(node_id.clone());
        }else{
            return None;
        }
    }

    fn _have_next(&self, node : &Node) -> bool {
        return self.graph.valid() && node.have_sons();
    }
}


/*function fixed_next!(path :: PathSolutionReader)
    node = PathGraph.get_node(path.graph, path.next_node_id)
    path.next_node_id = selected_next(path, node)

    #println("[$(path.step)] Selected: $(path.next_node_id.key)")
end

function selected_next(path :: PathSolutionReader, node :: Node) :: Union{NodeId,Nothing}
    if path.graph.valid
        if !isempty(node.sons)
            (node_id, edge_id) = first(node.sons)
            return node_id
        end

        return nothing
    else
        return nothing
    end
end
 */