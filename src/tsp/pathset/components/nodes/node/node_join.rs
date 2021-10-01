use crate::tsp::pathset::components::nodes::node::Node;

impl Node {
    pub fn join(&mut self, node_join : &Node){
        self._union_parents(node_join);
        self._union_sons(node_join);
        self.owners.union(&node_join.owners);
    }

    fn _union_parents(&mut self, node_join : &Node){
       for (parent_id, _edge_id) in node_join.parents.iter() {
            self.add_parent_id(parent_id);
       }
    }

    fn _union_sons(&mut self, node_join : &Node){
        for (son_id, _edge_id) in node_join.sons.iter() {
            self.add_son_id(son_id);
        }
    }
}