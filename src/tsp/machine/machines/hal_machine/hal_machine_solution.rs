use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::machine::machines::hal_machine::Grafo;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::alias::{Color, Km};
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::machine::components::timeline::timeline_cell::TimelineCell;

impl HamiltonianMachine {
    pub fn get_one_solution_graph(&self) -> Option<PathGraph> {
        let origin_cell = self.get_cell_origin();

        match origin_cell {
            Some(origin_cell) if !origin_cell.parents().is_empty() => {
                let parent_id = origin_cell.parents().iter().next().unwrap();

                let action_solution = self.db.get_action(parent_id).unwrap();
                let graph = action_solution.get_max_graph();

                return Some(graph.clone());
            }
            _ => None
        }
    }

    fn get_cell_origin(&self) -> Option<&TimelineCell> {
        let cell_origin = self.timeline.get_cell(&self.actual_km, &self.color_origin);
        
        println!("Have cell origin: {}",  cell_origin.is_some());
        return cell_origin;
    }
}

/* 
 get_one_solution_graph(machine :: IMachine) :: Union{Nothing, Graph}
    cell_origin = InterfaceMachine.get_cell_origin(machine)
    parents = cell_origin.parents

    if isempty(parents)
        return nothing
    else
        parent_1 = first(parents)
        action_solution = InterfaceMachine.get_action(machine, parent_1)
        graph = Actions.get_max_graph(action_solution)

        return graph
    end
end

*/