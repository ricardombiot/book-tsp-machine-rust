use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    id : NodeId,
    action_id : ActionId,
    color : Color,
    step : Step,

    parents : HashMap<NodeId, EdgeId>,
    sons : HashMap<NodeId, EdgeId>,

    owners : OwnersByStep
}

pub mod dict_node_edges;
pub mod node_parents_and_sons;
pub mod node_owners;
pub mod node_join;

impl Node {
    pub fn new_root(n : Color, b_max : Km, color : Color, owners_graph : &OwnersByStep, action_id : ActionId) -> Self {
        Node::_new(n, b_max, 0 as Step, color, owners_graph, action_id, None)
    }
    pub fn new(n : Color, b_max : Km, step : Step, color : Color, owners_graph : &OwnersByStep, action_id : ActionId, action_parent_id : ActionId) -> Self {
        Node::_new(n, b_max, step, color, owners_graph, action_id, Some(action_parent_id))
    }

    pub fn _new(n : Color, b_max : Km, step : Step, color : Color, owners_graph : &OwnersByStep, action_id : ActionId, action_parent_id : Option<ActionId>) -> Self {
        let parents  = HashMap::new();
        let sons = HashMap::new();
        let owners : OwnersByStep = owners_graph.derive();
        
        match action_parent_id {
            None => {
                let id = NodeId::new_root(n, b_max, action_id);
                let step = id.step();
                return Self {id, action_id, color, step, parents, sons, owners}
            }
            Some(action_parent_id) => {
                let id = NodeId::new(n, b_max, step, action_id, action_parent_id);
                return Self {id, action_id, color, step, parents, sons, owners}
            }
        }
    }

    pub fn is_root(&self) -> bool {
        self.id.is_root()
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn owners(&self) -> &OwnersByStep {
        &self.owners
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn step(&self) -> Step {
        self.step
    }

    /*
    pub fn join(&mut self, node_join : &Node){
        self._union_parents(node_join);
        self._union_sons(node_join);
        self.owners.union(&node_join.owners);
    }

    fn _union_parents(&mut self, node_join : &Node){
       for (parent_id, _edge_id) in node_join.parents.iter() {
            //self._add_parent(parent_id);
            self.add_parent_id(parent_id);
       }
    }

    fn _union_sons(&mut self, node_join : &Node){
        for (son_id, _edge_id) in node_join.sons.iter() {
            self.add_son_id(son_id);
        }
    }
    */

    /*
    pub fn add_parent(&mut self, parent : &Node) {
        self._add_parent(&parent.id)
    }

    fn _add_parent(&mut self, parent_id : & NodeId){
        if !self.have_parent(parent_id) {
            let origin_id = parent_id;
            let destine_id = &self.id;

            let edge_id = EdgeId::new(origin_id, destine_id);
            self.parents.insert(parent_id.clone(), edge_id);
        }
    }

    pub fn have_parent(&self, parent_id : &NodeId) -> bool{
       return self.parents.contains_key(&parent_id)
    }

    pub fn add_son(&mut self, son : &Node) {
        self._add_son(&son.id)
    }
    fn _add_son(&mut self, son_id : & NodeId){
        if !self.have_son(son_id) {
            let origin_id = &self.id;
            let destine_id = son_id; 

            let edge_id = EdgeId::new(origin_id, destine_id);
            self.sons.insert(son_id.clone(), edge_id);
        }
    }

    pub fn have_son(&self, son_id : &NodeId) -> bool{
       return self.sons.contains_key(&son_id)
    }


    pub fn delete_parent(&mut self, parent_id : &NodeId) {
        if self.have_parent(parent_id) {
            self.parents.remove(parent_id);
        }
    }

    pub fn delete_son(&mut self, son_id : &NodeId) {
        if self.have_son(son_id) {
            self.sons.remove(son_id);
        }
    }



    pub fn have_parents(&self) -> bool {
        !self.parents.is_empty()
    }

    pub fn have_sons(&self) -> bool {
        !self.sons.is_empty()
    }*/
}

