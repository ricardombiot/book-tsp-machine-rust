
use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
use crate::tsp::utils::alias::{Step, Color, Km};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::graph::path_graph::PathGraph;

impl PathSolutionReader {

    pub fn calc(&mut self){
        if self._next_step() {
            self.calc()
        }else{
            self._close_path();
        }
    }

    fn _is_finished(&self) -> bool {
        self.next_node_id.is_none()
    }

    fn _next_step(&mut self) -> bool {
        if self._is_finished() {
            return false;
        }else{
            self._push_step();
            self._fixed_next();
            self._clear_graph_by_owners();
            return true;
        }
    }

    fn _push_step(&mut self){
        let next_node_id = self.next_node_id.clone().unwrap();
        let next_action_id = next_node_id.action_id();

        let node = self.graph.table_nodes_by_action().get_node(&next_action_id, &next_node_id).unwrap();

        self.route.push(node.color());
        self.owners.push(&next_node_id);

        println!("[{}] Push step: {} ({})", self.step, next_node_id.key(), node.color());
        self.step += 1 as Step;
    }

    fn _close_path(&mut self){
        if self.is_origin_join {
            self.route.push(self.graph.color_origin());
            self.step += 1 as Step;
        }
    }
}

/*
function calc!(path :: PathSolutionReader)
    if next_step!(path)
        calc!(path)
    else
        close_path!(path)
    end
end

function close_path!(path :: PathSolutionReader)
    if !path.is_origin_join
        push!(path.route, path.graph.color_origin)
        path.step += 1
    end
end

function next_step!(path :: PathSolutionReader) :: Bool
    if path.next_node_id != nothing
        push_step!(path)
        fixed_next!(path)
        clear_graph_by_owners!(path)
        return true
    else
        return false
    end
end

function is_finished(path :: PathSolutionReader)
    path.next_node_id == nothing
end


function push_step!(path :: PathSolutionReader)
    node = PathGraph.get_node(path.graph, path.next_node_id)

    push!(path.route, node.color)
    Owners.push!(path.owners, path.step, path.next_node_id)

    #println("[$(path.step)] Push step: $(path.next_node_id.key) ($(node.color))")
    path.step += 1
end


*/