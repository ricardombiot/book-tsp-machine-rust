use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathGraph {

    pub fn join(&mut self, inmutable_graph_join : &PathGraph) -> bool{
        let is_valid = self._is_valid_join(&inmutable_graph_join);
        if is_valid {
            
            let graph_join = inmutable_graph_join;
            // ¿Why?
            //let graph_join = inmutable_graph_join.clone();

            //# $ O(N^3) $
            self._union_owners(graph_join);
            // # $ O(N^4) $
            self._join_nodes(graph_join);
            // # $ O(N^2) $
            self._join_lines(graph_join);
            // # $ O(N^3) $
            self._join_color_nodes(graph_join);
            //# $ O(N^4) $
            self._join_edges(graph_join);

            self._join_max_stages_review(graph_join);

            return true;
        }else{
            return false;
        }
    }

    fn _is_valid_join(&self, graph_join : &PathGraph) -> bool {
        let is_eq_step = self.next_step == graph_join.next_step;
        let is_eq_origin = self.color_origin == graph_join.color_origin;
        let is_both_valid = self.valid && graph_join.valid;
        let is_eq_action_parent = self.action_parent_id == graph_join.action_parent_id;

        return is_both_valid && is_eq_step && is_eq_origin && is_eq_action_parent
    }

    fn _union_owners(&mut self, graph_join : &PathGraph){
        self.owners_graph.union(&graph_join.owners_graph);
    }

    fn _join_nodes(&mut self, graph_join : &PathGraph){
        self.table_nodes_by_action.join(&graph_join.table_nodes_by_action);
    }

    fn _join_lines(&mut self, graph_join : &PathGraph){
        self.table_lines.join(&graph_join.table_lines);
    }

    fn _join_color_nodes(&mut self, graph_join : &PathGraph){
        self.table_color_nodes.join(&graph_join.table_color_nodes);
    }

    fn _join_edges(&mut self, graph_join : &PathGraph){
        self.table_edges.join(&graph_join.table_edges);
    }

    fn _join_max_stages_review(&mut self, graph_join : &PathGraph){
        self.max_review_stages = std::cmp::max(self.max_review_stages, graph_join.max_review_stages);
    }
}
