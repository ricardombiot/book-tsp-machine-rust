

use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::nodes::node::Node;

impl Node {

    pub fn add_parent(&mut self, parent : &Node) {
        self.add_parent_id(&parent.id)
    }

    pub fn add_parent_id(&mut self, parent_id : & NodeId){
        self.parents.insert(parent_id.clone());
    }

    pub fn have_parent(&self, parent : &Node) -> bool{
        return self.have_parent_id(parent.id())
    }
    pub fn have_parent_id(&self, parent_id : &NodeId) -> bool{
       return self.parents.contains(&parent_id)
    }

    pub fn add_son(&mut self, son : &Node) {
        self.add_son_id(&son.id)
    }
    pub fn add_son_id(&mut self, son_id : & NodeId){
        self.sons.insert(son_id.clone());
    }

    pub fn have_son(&self, son : &Node) -> bool{
        self.have_son_id(son.id())
    }

    pub fn have_son_id(&self, son_id : &NodeId) -> bool{
       return self.sons.contains(&son_id)
    }

    pub fn delete_parent(&mut self, parent_id : &NodeId) {
        //let have_it = self.have_parent_id(parent_id);
        //println!("In node {} Delete parent {} have [{}]" ,self.id().key(), parent_id.key(), have_it);

        self.parents.remove(parent_id);
        self.pop_owner(parent_id);
    }

    pub fn delete_son(&mut self, son_id : &NodeId) {
        //let have_it = self.have_son_id(son_id);
        //println!("In node {} Delete son {} have [{}]" ,self.id().key(), son_id.key(), have_it);

        self.sons.remove(son_id);
        self.pop_owner(son_id);
    }

    pub fn take_one_son(&self) -> Option<NodeId> {
        let mut result : Option<NodeId> = None;

        let value_first=  self.sons.iter().next();
        if value_first.is_some() {
           result = Some(value_first.unwrap().clone());
        }

        return result;
    }

    pub fn have_parents(&self) -> bool {
        !self.parents.is_empty()
    }

    pub fn have_sons(&self) -> bool {
        !self.sons.is_empty()
    }

    /* 
    pub fn parents_list(&self) -> Vec<(NodeId, EdgeId)> {
        self.parents.to_list()
    }

    pub fn sons_list(&self) -> Vec<(NodeId, EdgeId)> {
        self.sons.
    }*/

    pub fn parents_list(&self) -> Vec<NodeId> {
        let mut list : Vec<NodeId> = Vec::new();

        for node_id in self.parents.iter() {
            list.push(node_id.clone());
        }

        return list;
    }

    pub fn sons_list(&self) -> Vec<NodeId> {
        let mut list : Vec<NodeId> = Vec::new();

        for node_id in self.sons.iter() {
            list.push(node_id.clone());
        }

        return list;
    }
}