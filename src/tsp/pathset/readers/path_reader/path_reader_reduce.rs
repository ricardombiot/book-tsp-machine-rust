

use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
use crate::tsp::utils::alias::{Step, Color, Km};
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathSolutionReader {
    pub(super) fn _clear_graph_by_owners(&mut self){
        // elemino los no visitados
        let nodes_to_delete = self._save_to_delete_all_nodes_dont_selected_line();
        if nodes_to_delete > 0 {
            self.graph.apply_node_deletes();
            self.graph.review_owners_all_graph();
        }

    }

    fn _save_to_delete_all_nodes_dont_selected_line(&mut self) -> u32 {
        let mut nodes_to_delete = 0;
        let line = self.graph.table_lines().get(&self.step);
        
        match line {
            Some(line) => {
                let next_node_id = self.next_node_id.clone().unwrap();
                for node_id in line.clone() {
                    if !node_id.eq(&next_node_id) {
                        self.graph.save_to_delete(&node_id);
                        nodes_to_delete += 1;
                    }
                }
            },
            _ => ()
        }

        return nodes_to_delete;
    }
}

/*


function clear_graph_by_owners!(path :: PathSolutionReader)
    # Elimino los no visitados
    if remove_all_nodes_dont_selected_line(path)
        path.graph.required_review_ownwers = true
        PathGraph.review_owners_all_graph!(path.graph)
    end
end

function remove_all_nodes_dont_selected_line(path :: PathSolutionReader)
    nodes_to_delete = 0
    for node_id in PathGraph.get_line_nodes(path.graph, path.step)
        if node_id != path.next_node_id
            #println("[$(path.step)] Remove node in line... $(node_id.key) ")
            PathGraph.save_to_delete_node!(path.graph, node_id)
            nodes_to_delete += 1
        end
    end

    if nodes_to_delete > 0
        PathGraph.apply_node_deletes!(path.graph)
        return true
    else
        return false
    end
end

*/