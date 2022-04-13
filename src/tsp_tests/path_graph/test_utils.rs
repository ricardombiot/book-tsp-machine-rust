
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::utils::alias::{Step};
use crate::tsp::pathset::components::nodes::node_id::{NodeId, NodesIdSet};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::pathset::components::nodes::node::dict_edgeid_by_nodeid::DictEdgeIdByNodeId;
use crate::tsp::utils::generator_ids;

pub fn should_be_only_node_id(set_nodes : &NodesIdSet, expected_node_id : &NodeId){
    // Only should be root node
    assert_eq!(set_nodes.len(), 1);
    let root_node_id = set_nodes.iter().next().unwrap();
    //println!("{:?}", root_node_id);
    let ok_root_id = root_node_id.eq(expected_node_id);
    assert!(ok_root_id);
}

/* 
pub fn check_edge(graph : &PathGraph, origin_id: &NodeId, destine_id : &NodeId) -> EdgeId {
    let id_edge = EdgeId::new(origin_id, destine_id);

    let edge = graph.table_edges().get(&id_edge).unwrap();
    assert_eq!(edge.id().destine_id(), destine_id);
    assert_eq!(edge.id().origin_id(), origin_id);

    return id_edge;
}*/


pub fn check_have_owners(owners_to_check : &OwnersByStep, step: Step, list: Vec<&NodeId>){
    let owners_set = owners_to_check.get_step_owners(step).unwrap();

    for node_id in list {
        assert!(owners_set.have(node_id.key()));
    }
}

pub fn check_list_by_nodeid(list: Vec<NodeId>, list_keys : Vec<&NodeId>){
    let mut set = NodesIdSet::new();

    for node_id in list {
        set.insert(node_id.clone());
    }

    for node_id in list_keys {
        assert!(set.contains(node_id));
        set.remove(node_id);
    }

    assert!(set.is_empty());
}

pub fn check_dict_edges_by_nodeid(dict : &DictEdgeIdByNodeId, list_keys : Vec<(&NodeId, &EdgeId)>){
    let mut dict = dict.clone();
    for (node_id, edge_id) in list_keys {
        assert!(dict.have(node_id));

        let value_edge_id = dict.get(node_id).unwrap();

        assert!(edge_id.eq(value_edge_id));
        dict.delete(node_id);
    }
    assert!(dict.is_empty())

}

pub fn check_set_nodes(set_nodes: &NodesIdSet, list_keys : Vec<&NodeId>){
    let mut set_nodes = set_nodes.clone();
    for node_id in list_keys {
        assert!(set_nodes.remove(node_id));
    }
    assert!(set_nodes.is_empty())
}