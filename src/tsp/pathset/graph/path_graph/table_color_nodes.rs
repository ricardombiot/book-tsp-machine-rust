
use crate::tsp::utils::alias::{Color};
use crate::tsp::pathset::components::nodes::node_id::NodesIdSet;
use crate::tsp::utils::inmutable_dict::{DictInmutableWapper, InmutableDict, InmutableDictCommons};
use crate::tsp::pathset::components::nodes::node::Node;

#[derive(Clone)]
pub struct TableColorNodes {
    table : InmutableDict<Color, NodesIdSet>
}

impl TableColorNodes {
    pub fn new() -> Self{
        let table = InmutableDict::new();
        TableColorNodes{table}
    }

    pub fn add_node_color(&mut self, node : &Node){
        let color = node.color();
        self._if_not_exist_init_set(node.color());

        let mut set_colors = self.table.get_mut(&color).unwrap();
        set_colors.insert(node.id().clone());
        /* 
        let mut set_colors = self.pop(&color).unwrap();
        set_colors.insert(node.id().clone());
        self.put(color, set_colors);*/
    }

    fn _if_not_exist_init_set(&mut self, color : Color){
        let set_colors = NodesIdSet::new();
        self.put(color , set_colors);
    }
}

impl InmutableDictCommons<Color, NodesIdSet> for TableColorNodes {
    fn dict(&self) -> & InmutableDict<Color, NodesIdSet>  {
        &self.table
    }

    fn dict_mut(&mut self) -> &mut InmutableDict<Color, NodesIdSet>  {
        &mut self.table
    }

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<Color, NodesIdSet>  {
        &mut self.table
    }
}
