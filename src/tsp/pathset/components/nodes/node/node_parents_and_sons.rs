use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;

impl Node {

    /*
    pub fn for_each_parent<'a,F>(&'a mut self, mut func : F) where F : FnMut(&'a mut Node, &NodeId, &EdgeId) {
        //let pointer : &&mut Node = &mut &self;
        let pointer : &'a mut Node = self;
        for (node_id, edge_id) in pointer.parents.iter() {
            func(pointer, node_id, edge_id)
        }
    }*/

    pub fn add_parent(&mut self, parent : &Node) {
        self.add_parent_id(&parent.id)
    }

    pub fn add_parent_id(&mut self, parent_id : & NodeId){
        if !self.have_parent_id(parent_id) {
            let origin_id = parent_id;
            let destine_id = &self.id;

            let edge_id = EdgeId::new(origin_id, destine_id);
            self.parents.insert(parent_id.clone(), edge_id);
        }
    }

    pub fn have_parent(&self, parent : &Node) -> bool{
        return self.have_parent_id(parent.id())
    }
    pub fn have_parent_id(&self, parent_id : &NodeId) -> bool{
       return self.parents.contains_key(&parent_id)
    }

    pub fn add_son(&mut self, son : &Node) {
        self.add_son_id(&son.id)
    }
    pub fn add_son_id(&mut self, son_id : & NodeId){
        if !self.have_son_id(son_id) {
            let origin_id = &self.id;
            let destine_id = son_id; 

            let edge_id = EdgeId::new(origin_id, destine_id);
            self.sons.insert(son_id.clone(), edge_id);
        }
    }

    pub fn have_son(&self, son : &Node) -> bool{
        self.have_son_id(son.id())
    }

    pub fn have_son_id(&self, son_id : &NodeId) -> bool{
       return self.sons.contains_key(&son_id)
    }

    pub fn delete_parent(&mut self, parent_id : &NodeId) {
        if self.have_parent_id(parent_id) {
            self.parents.remove(parent_id);
        }
    }

    pub fn delete_son(&mut self, son_id : &NodeId) {
        if self.have_son_id(son_id) {
            self.sons.remove(son_id);
        }
    }

    pub fn have_parents(&self) -> bool {
        !self.parents.is_empty()
    }

    pub fn have_sons(&self) -> bool {
        !self.sons.is_empty()
    }

}