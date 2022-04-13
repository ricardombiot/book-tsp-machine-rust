
use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
use crate::tsp::utils::alias::Step;

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

            //self._save_graph_as_png();

            return true;
        }
    }

    fn _save_graph_as_png(&self){
        let name = format!("graph_{}", self.step);
        let _res = self.graph.to_png(name, None);
    }

    fn _push_step(&mut self){
        let next_node_id = self.next_node_id.clone().unwrap();
        let next_action_id = next_node_id.action_id();

        let node = self.graph.table_nodes_by_action().get_node(&next_action_id, &next_node_id).unwrap();

        self.route.push(node.color());
        self.owners.push(&next_node_id);

        //println!("[{}] Push step: {} ({})", self.step, next_node_id.key(), node.color());
        self.step += 1 as Step;
    }

    fn _close_path(&mut self){
        if !self.is_origin_join {
            self.route.push(self.graph.color_origin());
            self.step += 1 as Step;
        }
    }
}
