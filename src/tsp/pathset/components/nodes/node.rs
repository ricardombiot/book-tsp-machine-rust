use crate::tsp::pathset::components::edges::edge::Edge;
use crate::tsp::utils::alias::{Color, Km, Step, ActionId};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Node<'a,'b> {
    id : NodeId,
    action_id : ActionId,
    color : Color,
    step : Step,

    parents : HashMap<&'a NodeId, EdgeId<'a,'b>>,
    sons : HashMap<&'a NodeId, EdgeId<'a,'b>>,

    owners : OwnersByStep
}

impl<'a, 'b> Node<'a, 'a> {
    pub fn new_root(n : Color, b_max : Km, color : Color, owners_graph : OwnersByStep, action_id : ActionId) -> Self {
        Node::_new(n, b_max, 0 as Step, color, owners_graph, action_id, None)
    }
    pub fn new(n : Color, b_max : Km, step : Step, color : Color, owners_graph : OwnersByStep, action_id : ActionId, action_parent_id : ActionId) -> Self {
        Node::_new(n, b_max, step, color, owners_graph, action_id, Some(action_parent_id))
    }

    pub fn _new(n : Color, b_max : Km, step : Step, color : Color, owners_graph : OwnersByStep, action_id : ActionId, action_parent_id : Option<ActionId>) -> Self {
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

    pub fn join(&'a mut self, node_join : &Node<'b,'b>){
        self._union_parents(node_join);
        //self._union_sons(node_join);
    }

    fn _union_parents(&'a mut self, node_join : &Node<'b, 'b>){
       for (parent_id, _edge_id) in node_join.parents.iter() {
            self._add_parent(parent_id);
       }
    }
    /*
    fn _union_sons<'b>(&'a mut self, node_join : &Node<'b>){
        for (son_id, _edge_id) in node_join.sons.iter() {
            let son_id : &'a NodeId = &son_id;
            self._add_son(son_id);
        }
     }*/

    fn _add_parent(&'a mut self, parent_id : &'b NodeId){
        if !self.have_parent(parent_id) {
            let destine_id : &'a NodeId = &self.id;
            let edge_id = EdgeId::new(parent_id, destine_id);
            self.parents.insert(parent_id, edge_id);
        }
    }

    fn have_parent(&self, parent_id : &NodeId) -> bool{
       return self.parents.contains_key(&parent_id)
    }

    /*
    fn _add_son(&'a mut self, son_id : &'a NodeId){
        if !self.have_son(son_id) {
            let origin_id : &'a NodeId = &self.id;
            let edge_id = EdgeId::new(origin_id, son_id);
            self.parents.insert(son_id, edge_id);
        }
    }

    fn have_son(&self, son_id : &NodeId) -> bool{
       return self.sons.contains_key(&son_id)
    }*/


    /*
    
function join!(node :: Node, node_join :: Node)
    union_parents!(node, node_join)
    union_sons!(node, node_join)
    Owners.union!(node.owners, node_join.owners)
end

# O(N)
function union_parents!(node :: Node, node_join :: Node)
    for (parent_id, edge_id) in node_join.parents
        if !haskey(node.parents, parent_id)
            node.parents[parent_id] = edge_id
        end
    end
end

# O(N)
function union_sons!(node :: Node, node_join :: Node)
    for (son_id, edge_id) in node_join.sons
        if !haskey(node.sons, son_id)
            node.sons[son_id] = edge_id
        end
    end
end

    */

}