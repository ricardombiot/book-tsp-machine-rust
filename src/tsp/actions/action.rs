use std::fmt;
use crate::tsp::utils::{alias::{Color, Km, ActionId, ActionsIdSet, Step}, generator_ids, inmutable_dict::InmutableDictCommons};
use crate::tsp::actions::table_graph_by_length::TableGraphByLenght;
use crate::tsp::pathset::graph::path_graph::PathGraph;

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Action {
    id : ActionId,
    km : Km,
    up_color : Color,

    props_parents : ActionsIdSet,
    props_graph : TableGraphByLenght,
    max_length_graph : Step,
    // Is valid after add a valid graph
    valid : bool,
    was_execute: bool 
}

impl Action {

    pub fn new_up(n: usize, km: u32, up_color: usize, parents: ActionsIdSet) -> Action {
        let mut action = _new(n, km,  up_color);
        action.props_parents = parents;

        return action;
    }

    pub fn new_init(n : Color, b_max: Km, up_origin_color: Color) -> Self { 
        let km : Km = 0 as Km;
        let mut action = _new(n, km, up_origin_color);

        action.max_length_graph = 0 as Step;

        let graph_init = PathGraph::new(n, b_max, up_origin_color, action.id());
        action.push_graph_by_lenght(graph_init);
        action.was_execute = true;

        return action;
    }

    pub fn fixed_as_executed(&mut self){
        self.was_execute = true;
    }


    pub fn push_graph_by_lenght(&mut self, graph_join : PathGraph){
        let lenght = graph_join.get_lenght();
        if lenght > self.max_length_graph {
            self.max_length_graph = lenght;
        }

        self.props_graph.push_graph_by_lenght(graph_join);
    }

    pub fn was_execute(&self) -> bool {
        self.was_execute
    }

    pub fn id(&self) -> ActionId {
        self.id
    }

    pub fn km(&self) -> Km {
        self.km
    }

    pub fn up_color(&self) -> Color {
        self.up_color
    }

    pub fn props_parents(&self) -> &ActionsIdSet{
        &self.props_parents
    }
   
    pub fn props_graph(&self) -> &TableGraphByLenght {
        &self.props_graph
    }

    pub fn max_length_graph(&self) -> Step {
        self.max_length_graph
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn get_max_graph(&self) -> &PathGraph {
        return self.props_graph.get(&self.max_length_graph).unwrap();
    }

}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Action( ID:{} KM: {} COLOR: {} )", self.id, self.km, self.up_color);
    }
}


fn _new(n : Color, km: Km, up_color: Color) -> Action { 
    let id = generator_ids::get_action_id(n, km, up_color);
    let props_parents = ActionsIdSet::new();
    let props_graph = TableGraphByLenght::new();
    let max_length_graph = 0;
    let valid = true;
    let was_execute = false;
    Action {id, km, up_color, props_parents, props_graph, max_length_graph, valid ,was_execute} 
}
