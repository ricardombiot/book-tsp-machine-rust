use crate::tsp::utils::alias::{Color, Km, Step , ActionId, UniqueNodeKey, InfoActionId};
use crate::tsp::utils::generator_ids;
use crate::tsp::utils::generator_node_key;
use crate::tsp::pathset::components::edges::edge::Edge;
use crate::tsp::pathset::components::nodes::node_id::NodeId;

#[test]
pub fn test_create_edge_checking_ids(){
    let n: Color = 5;
    let b_max = 100;
    let km_origin : Km = 1;
    let color_origin : Color = 3;

    let action_id : ActionId = generator_ids::get_action_id(n, km_origin, color_origin);
    let action_parent_id = action_id;

    let node_origin_id = NodeId::new_root(n, b_max, action_id);
    let key_expected : UniqueNodeKey = generator_node_key::calc_unique_node_key(n, b_max, 0 as Step, action_id, action_parent_id);
    assert_eq!(node_origin_id.key(), key_expected);


    let destine_id : ActionId = generator_ids::get_action_id(n, 5 as Km, 2 as Color);
    
    let action_id = destine_id;
    
    let node_destine_id : NodeId = NodeId::new(n, b_max, 1 as Step, action_id, action_parent_id);
    let key_expected : UniqueNodeKey = generator_node_key::calc_unique_node_key(n, b_max, 1 as Step, action_id, action_parent_id);
    assert_eq!(node_destine_id.key(), key_expected);
    
    
    let edge = Edge::new(&node_origin_id, &node_destine_id);

    let ok_origin = edge.id().origin_id().eq(&node_origin_id);
    let ok_destine = edge.id().destine_id().eq(&node_destine_id);

    assert!(ok_origin);
    assert!(ok_destine);

    let origin_info_expected : InfoActionId = (1 as Km, 3 as Color);
    let destine_info_expected : InfoActionId = (5 as Km, 2 as Color);

    let restore_origin_id = edge.id().origin_id();

    let (destine_info, origin_info) = restore_origin_id.get_info_node_id(n);
    assert_eq!(destine_info, origin_info);
    assert_eq!(destine_info, origin_info_expected);
    assert_eq!(origin_info, origin_info_expected);


    let restore_destine_id = edge.id().destine_id();

    let (destine_info, origin_info) = restore_destine_id.get_info_node_id(n);
    assert_eq!(destine_info, destine_info_expected);
    assert_eq!(origin_info, origin_info_expected);

}