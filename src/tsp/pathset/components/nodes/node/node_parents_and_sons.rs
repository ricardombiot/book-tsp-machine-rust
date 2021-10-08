
use crate::tsp::pathset::components::edges::edge::Edge;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl Node {

    pub fn add_parent(&mut self, parent : &Node) {
        self.add_parent_id(&parent.id)
    }

    pub fn add_parent_id(&mut self, parent_id : & NodeId){
        let origin_id = parent_id;
        let destine_id = &self.id;
        let key = parent_id;

        self.parents.add_edge_id(origin_id, destine_id, key);
    }

    pub fn have_parent(&self, parent : &Node) -> bool{
        return self.have_parent_id(parent.id())
    }
    pub fn have_parent_id(&self, parent_id : &NodeId) -> bool{
       return self.parents.have(&parent_id)
    }

    pub fn add_son(&mut self, son : &Node) {
        self.add_son_id(&son.id)
    }
    pub fn add_son_id(&mut self, son_id : & NodeId){
        let origin_id = &self.id;
        let destine_id = son_id;
        let key = son_id;

        self.sons.add_edge_id(origin_id, destine_id, key);
    }

    pub fn have_son(&self, son : &Node) -> bool{
        self.have_son_id(son.id())
    }

    pub fn have_son_id(&self, son_id : &NodeId) -> bool{
       return self.sons.have(&son_id)
    }

    pub fn delete_parent(&mut self, parent_id : &NodeId) {
        self.parents.delete_by_id(parent_id);
        self.pop_owner(parent_id);
    }

    pub fn delete_son(&mut self, son_id : &NodeId) {
        self.sons.delete_by_id(son_id);
        self.pop_owner(son_id);
    }

    pub fn take_one_son(&self) -> Option<(NodeId,EdgeId)> {
        self.sons.take_one()
    }

    pub fn have_parents(&self) -> bool {
        !self.parents.is_empty()
    }

    pub fn have_sons(&self) -> bool {
        !self.sons.is_empty()
    }

    pub fn parents_list(&self) -> Vec<(NodeId, EdgeId)> {
        self.parents.to_list()
    }

    pub fn sons_list(&self) -> Vec<(NodeId, EdgeId)> {
        self.sons.to_list()
    }

}