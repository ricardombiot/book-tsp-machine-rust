use crate::tsp::utils::alias::{Color, Km, Step , ActionId};
use crate::tsp::utils::generator_ids;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::nodes::node::dict_edgeid_by_nodeid::DictEdgeIdByNodeId;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;


#[test]
pub fn test_inmutable_dict(){
    let n : Color = 10;
    let b_max : Km = 20;
    let owners_graph : OwnersByStep = OwnersByStep::seed(n, b_max);

    let node_root = Node::new_root(n,b_max,0 as Color, &owners_graph, 1 as ActionId);
    let action_parent_id = 1 as ActionId;
    let action_id : ActionId = generator_ids::get_action_id(n, 2 as Km, 3 as Color);
    let node_s1_3 = Node::new(n, b_max, 1 as Step, 3 as Color, &owners_graph, action_id, action_parent_id);


    let mut parents : DictEdgeIdByNodeId = DictEdgeIdByNodeId::new();
    parents.add_edge_id(node_root.id(), node_s1_3.id(), node_root.id());

    assert_eq!(parents.is_empty(),false);

    // The target is be able to iter and mutable the dictionary without problems.
    for (node_origin, _edge_id) in parents.to_list().iter(){
        parents.delete(node_origin);
    }

    assert_eq!(parents.is_empty(),true);

}