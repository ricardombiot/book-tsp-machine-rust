
use crate::tsp::utils::alias::{Color, Km, Step , ActionId};
use crate::tsp::utils::generator_ids;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;

#[test]
pub fn test_node_root(){
    let n : Color = 10;
    let b_max : Km = 20;
    let owners_graph : OwnersByStep = OwnersByStep::seed(n, b_max);

    let node_root = Node::new_root(n,b_max,0 as Color, &owners_graph, 1 as ActionId);

    assert_eq!(node_root.id().action_id(), 1 as ActionId);
    assert_eq!(node_root.id().action_parent_id(), 1 as ActionId);
    assert_eq!(node_root.color(), 0 as Color);
    assert_eq!(node_root.step(), 0 as Step);
    assert_eq!(node_root.have_parents(), false);
    assert_eq!(node_root.have_sons(), false);

    assert_eq!(node_root.is_root(), true);
}


#[test]
pub fn test_node_add_parent_and_son(){
    let n : Color = 10;
    let b_max : Km = 20;
    let owners_graph : OwnersByStep = OwnersByStep::seed(n, b_max);

    let mut node_root = Node::new_root(n,b_max,0 as Color, &owners_graph, 1 as ActionId);

    assert_eq!(node_root.id().action_id(), 1 as ActionId);
    assert_eq!(node_root.id().action_parent_id(), 1 as ActionId);
    assert_eq!(node_root.color(), 0 as Color);
    assert_eq!(node_root.step(), 0 as Step);
    assert_eq!(node_root.have_parents(), false);
    assert_eq!(node_root.have_sons(), false);

    assert_eq!(node_root.is_root(), true);

    let action_parent_id = 1 as ActionId;
    let action_id : ActionId = generator_ids::get_action_id(n, 2 as Km, 3 as Color);
    let mut node_s1_3 = Node::new(n, b_max, 1 as Step, 3 as Color, &owners_graph, action_id, action_parent_id);

    assert_eq!(node_s1_3.id().action_id(), action_id);
    assert_eq!(node_s1_3.id().action_parent_id(), 1 as ActionId);
    assert_eq!(node_s1_3.color(), 3 as Color);
    assert_eq!(node_s1_3.step(), 1 as Step);
    assert_eq!(node_s1_3.have_parents(), false);
    assert_eq!(node_s1_3.have_sons(), false);

    assert_eq!(node_s1_3.is_root(), false);


    assert_eq!(node_root.have_son(&node_s1_3), false);
    node_root.add_son(&node_s1_3);
    assert_eq!(node_root.have_parents(), false);
    assert_eq!(node_root.have_sons(), true);
    assert_eq!(node_root.have_son(&node_s1_3), true);


    assert_eq!(node_s1_3.have_parent(&node_root), false);
    node_s1_3.add_parent(&node_root);
    assert_eq!(node_s1_3.have_parents(), true);
    assert_eq!(node_s1_3.have_sons(), false);
    assert_eq!(node_s1_3.have_parent(&node_root), true);


    assert_eq!(node_root.have_son(&node_s1_3), true);
    node_root.delete_son(node_s1_3.id());
    assert_eq!(node_root.have_son(&node_s1_3), false);
    

    let list_parents = node_s1_3.parents_list();
    assert!(!list_parents.is_empty());
    //println!("{:#?}", list_parents);

    assert_eq!(node_s1_3.have_parent(&node_root), true);
    node_s1_3.delete_parent(node_root.id());
    assert_eq!(node_s1_3.have_parent(&node_root), false);

    let list_parents = node_s1_3.parents_list();
    assert!(list_parents.is_empty());



    /*
    node_s1_3.for_each_parent(|pointer : &mut Node, node_id, edge_id| {
        println!("{}", edge_id);

        //pointer.parents.
        //let p : &mut Node = *pointer;
        pointer.delete_parent(node_id);
    });
    */   
}