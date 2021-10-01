use crate::tsp::pathset::components::nodes::node::Node;

impl Node {

    pub fn push_owner(&mut self, node_owner : &Node){
        self.owners.push(node_owner.id());
    }

    pub fn pop_owner(&mut self, node_owner : &Node){
        self.owners.pop(node_owner.id());
    }

    pub fn have_owner(&self, node_owner : &Node) -> bool {
        self.owners.have(node_owner.id())
    }

    pub fn intersect_owners(&mut self, node_owner : &Node) {
        self.owners.intersect_quick(node_owner.owners());
    }

}