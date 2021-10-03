use crate::tsp::utils::alias::{Color, Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::pathset::components::nodes::node::Node;


impl PathGraph  {
    pub(super) fn _init(&mut self, action_id : ActionId){
        let node = self._new_node(self.color_origin, action_id);
        self._add_line();
        self.next_step += 1 as Step;
        self.action_parent_id = Some(action_id);
        
        self._add_node(node);
    }

    pub(super) fn _new_node(&self, color_up : Color, action_id : ActionId) -> Node {
        return Node::create(self.n,self.b_max, self.next_step, color_up, &self.owners_graph, action_id, self.action_parent_id);
    }

    pub(super) fn _add_line(&mut self){
        self.table_lines.add_line(self.next_step);
    }

}

