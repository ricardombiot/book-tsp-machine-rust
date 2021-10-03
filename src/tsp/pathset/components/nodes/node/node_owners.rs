use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;


impl Node {

    pub fn push_owner_myself(&mut self) {
        let id = self.id().clone();
        self.push_owner(&id);
    }

    pub fn push_owner(&mut self, id : &NodeId){
        self.owners.push(&id)
    }

    pub fn pop_owner(&mut self, id : &NodeId){
        self.owners.pop(&id);
    }

    pub fn have_owner(&self, id : &NodeId) -> bool {
        self.owners.have(id)
    }

    pub fn intersect_owners(&mut self, owners_b : &OwnersByStep) {
        self.owners.intersect_quick(owners_b);
    }

    /*
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
    }*/

    

}