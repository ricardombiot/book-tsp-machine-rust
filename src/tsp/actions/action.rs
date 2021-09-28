use std::fmt;
use std::collections::HashMap;
use crate::tsp::utils::{alias::{Color, Km, ActionId, ActionsIdSet, Step}, generator_ids};

pub struct PathGraph;

pub type TablePathGraphsByLenght = HashMap<Step, Box<PathGraph>>;

pub struct Action {
    id : ActionId,
    km : Km,
    up_color : Color,

    props_parents : ActionsIdSet,
    props_graph : TablePathGraphsByLenght,
    max_length_graph : Step,
    // Is valid after add a valid graph
    valid : bool
}

impl Action {
    pub fn new_init(n : Color, _b_max: Km, up_origin_color: Color) -> Self { 
        let km : Km = 0 as Km;
        return Action::_new(n, km,up_origin_color)
    }

    pub fn new_up(n: usize, km: u32, up_color: usize, parents: ActionsIdSet) -> Action {
        let mut action = Action::_new(n, km,  up_color);
        action.props_parents = parents;

        //....
        return action;
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
   
    pub fn props_graph(&self) -> &TablePathGraphsByLenght {
        &self.props_graph
    }

    pub fn max_length_graph(&self) -> Step {
        self.max_length_graph
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
}

impl Action {
    fn _new(n : Color, km: Km, up_color: Color) -> Self { 
        let id = generator_ids::get_action_id(n, km, up_color);
        let props_parents = ActionsIdSet::new();
        let props_graph = TablePathGraphsByLenght::new();
        let max_length_graph = 0;
        let valid = true;
        Self {id, km, up_color, props_parents, props_graph, max_length_graph, valid } 
    }
}


impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Action( ID:{} KM: {} COLOR: {} )", self.id, self.km, self.up_color);
    }
}