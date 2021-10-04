use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl Node {
    pub fn join(&mut self, node_join : &Node){
        self._union_parents(node_join);
        self._union_sons(node_join);
        self.owners.union(&node_join.owners);
    }

    fn _union_parents(&mut self, node_join : &Node){
        self.parents.join(&node_join.parents);
    }

    fn _union_sons(&mut self, node_join : &Node){
        self.sons.join(&node_join.sons);
    }
}