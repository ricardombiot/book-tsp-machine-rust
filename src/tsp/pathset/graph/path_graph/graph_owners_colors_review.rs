use crate::tsp::utils::alias::{Color, Step, SetColors};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {

    pub(super) fn _filter_by_incoherence_colors(&self, node_id : &NodeId) -> bool{
        let mut should_be_filter = false;
        let mut set_of_all_colors = SetColors::new();
        let mut set_conflict_colors = SetColors::new();
        
        let next_step: Step = self.next_step;
        //@Review if should add the -1
        //  for step in Step(0):Step(graph.next_step-1)
        //# $ O(Step) = O(N) $
        for step in 0..next_step {
            //# $ O(N^2) $
            let colors_step = self._load_all_colors_node_step_at_review_owners(&step, node_id);
        
            if self._controller_incoherence_fixed_color_in_more_than_one_step(&step, &mut set_conflict_colors, &colors_step) {
                should_be_filter = true;
            }else if self._controller_incoherence_enough_color(&step, &mut set_of_all_colors, &colors_step) {
                should_be_filter = true;
            }

            if should_be_filter {
                return true;
            }
        }


        return should_be_filter;
    }

    fn _controller_incoherence_enough_color(&self, step : &Step, set_of_all_colors : &mut SetColors, colors_step : &SetColors ) -> bool {
        let mut should_be_filter = false;
        let n = self.n as u32;

        // # $ O(N) $ UNION
        for color in colors_step.iter(){
            set_of_all_colors.insert(color.clone());
        } 

        let number_of_possible_colors = set_of_all_colors.len() as Color;
        //# I shouldn´t filter the return to origin node
        let number_color_required_step = std::cmp::min(n,step+1) as Color;
        if number_of_possible_colors < number_color_required_step  {
            should_be_filter = true;
        }

        return should_be_filter;
    }

    fn _controller_incoherence_fixed_color_in_more_than_one_step(&self, step : &Step, set_conflict_colors : &mut SetColors, colors_step : &SetColors ) -> bool {
        let mut should_be_filter = false;
        let n = self.n as u32;


        if colors_step.len() == 1 && n > step+1 {
            //# $ O(N) $
            let fixed_color_in_step = colors_step.iter().next().unwrap();

            let is_fixed_color_in_previus_step = set_conflict_colors.contains(fixed_color_in_step);
            if is_fixed_color_in_previus_step {
                should_be_filter = true;
            }else{
                set_conflict_colors.insert(fixed_color_in_step.clone());
            }
        }

        return should_be_filter;
    }

    fn _load_all_colors_node_step_at_review_owners(&self, step : &Step, node_id: &NodeId) -> SetColors{
        let mut colors_step = SetColors::new();

        let action_id = node_id.action_id();
        let node = self.table_nodes_by_action.get_node(&action_id, node_id).unwrap();
        let owners_node = node.owners();

        let set_nodes = self.table_lines.get(&step).unwrap();
        for line_node_id in set_nodes.iter() {
            let action_id = line_node_id.action_id();
            let line_node = self.table_nodes_by_action.get_node(&action_id, line_node_id).unwrap();

            if owners_node.have(line_node_id) && line_node.is_valid() {
                colors_step.insert(line_node.color());
            }
        }

        return colors_step;
    }

}


/*
# O(N^2)
function load_all_colors_node_step_at_review_owners(graph :: Graph, step :: Step, node :: Node) :: SetColors
    colors :: SetColors = SetColors()

    # $ O(N^2) $
    for node_id in graph.table_lines[step]
        if Owners.have(node.owners, step, node_id)
            node_owner = PathGraph.get_node(graph, node_id)

            if node_owner.owners.valid
                push!(colors, node_owner.color)
            end
        end
    end

    return colors
end

# $ O(N) $
function controller_incoherence_enough_color!(n :: Color, step :: Step, set_of_all_colors :: SetColors,  colors_step :: SetColors) :: Bool
    # $ O(N) $
    union!(set_of_all_colors, colors_step)
    number_of_possible_colors = length(set_of_all_colors)
    # I shouldn´t filter the return to origin node
    number_color_required_step = Int64(min(n, step+1))
    if number_of_possible_colors < number_color_required_step
        return true
    else
        return false
    end
end

# $ O(N) $
function controller_incoherence_fixed_color_in_more_than_one_step!(n :: Color, step :: Step, set_conflict_colors :: SetColors,  colors_step :: SetColors) :: Bool
    if length(colors_step) == 1 && n > step+1
        # $ O(N) $
        if issubset(colors_step, set_conflict_colors)
            return true
        else
        # $ O(N) $
            union!(set_conflict_colors, colors_step)
        end
    end

    return false
end

*/