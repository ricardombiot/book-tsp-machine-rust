

pub trait IMachine {
    fn get_cell_origin(&self) ;
    
    fn get_one_solution_graph(&self) -> Option<PathGraph> {

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