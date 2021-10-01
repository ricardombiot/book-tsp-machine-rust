use crate::tsp::utils::alias::{Color, Km, Step , ActionId, UniqueNodeKey, InfoActionId};
use crate::tsp::pathset::components::nodes::node_id::NodeId;

#[test]
pub fn test_get_info_node_id_root(){
    let n : Color = 4;
    let b_max : Km = 10;
    let step : Step = 0;
    let action_id : ActionId = 1;
    let destine_info_expected : InfoActionId = (0,0);
    let origin_info_expected : InfoActionId = (0,0);

    let node_id = NodeId::new_root(n, b_max, action_id);

    let (destine_info, origin_info) = node_id.get_info_node_id(n);

    assert_eq!(node_id.key(), 1 as UniqueNodeKey); 
    assert_eq!(node_id.step(), step); 
    assert_eq!(node_id.action_id(), action_id); 
    assert_eq!(node_id.action_parent_id(), action_id); 
   
    assert_eq!(destine_info, destine_info_expected); 
    assert_eq!(origin_info, origin_info_expected); 
}


#[test]
pub fn test_get_info_node_id(){
    let n : Color = 4;
    let b_max : Km = 10;
    let step : Step = 1;
    let action_id : ActionId = 6;
    let action_parent_id : ActionId = 1;
    let destine_info_expected : InfoActionId = (1,1);
    let origin_info_expected : InfoActionId = (0,0);

    let node_id = NodeId::new(n, b_max, step, action_id, action_parent_id);

    let (destine_info, origin_info) = node_id.get_info_node_id(n);

    assert_eq!(node_id.key(), 1606 as UniqueNodeKey); 
    assert_eq!(node_id.step(), step); 
    assert_eq!(node_id.action_id(), action_id); 
    assert_eq!(node_id.action_parent_id(), action_parent_id); 
   
    assert_eq!(destine_info, destine_info_expected); 
    assert_eq!(origin_info, origin_info_expected); 
}

/*
function test_get_info_node_id()
    n= Color(4)
    b= Km(10)

    node_id = NodeIdentity.new(n, b, Step(0), ActionId(1), nothing)
    (info, info_parent) = NodeIdentity.get_info_node_id(n, node_id)

    @test node_id.key == UniqueNodeKey(1)
    @test info == (Km(0), Color(0))
    @test info_parent == (Km(0), Color(0))


    node_id = NodeIdentity.new(n, b, Step(1), ActionId(6), ActionId(1))
    (info, info_parent) = NodeIdentity.get_info_node_id(n, node_id)

    @test node_id.key == UniqueNodeKey(1606)
    @test info == (Km(1), Color(1))
    @test info_parent == (Km(0), Color(0))

    node_id = NodeIdentity.new(n, b, Step(2), ActionId(10), ActionId(1))
    (info, info_parent) = NodeIdentity.get_info_node_id(n, node_id)

    @test node_id.key == UniqueNodeKey(3210)
    @test info == (Km(2), Color(1))
    @test info_parent == (Km(0), Color(0))
end*/