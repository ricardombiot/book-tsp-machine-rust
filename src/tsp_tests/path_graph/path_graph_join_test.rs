

use crate::tsp::utils::alias::{Color, Km, Step};
use crate::tsp::pathset::components::nodes::node_id::{NodeId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::utils::generator_ids;
use crate::tsp_tests::path_graph::test_utils::{check_edge,check_set_nodes, check_dict_edges_by_nodeid, check_have_owners};


/*
    0
  2   4
  4   2
  6   6
*/
#[test]
fn test_build_join(){
    build_join();
}



fn build_join() -> PathGraph {
    let n : Color = 10 as Color;
    let b_max = 10 as Km;
    let color_origin = 0 as Color;

    let action_id_s0_0 = generator_ids::get_action_id(n, 0 as Km, 0 as Color); 

    let action_id_s1_2 = generator_ids::get_action_id(n, 1 as Km, 2 as Color); 
    let action_id_s1_4 = generator_ids::get_action_id(n, 1 as Km, 4 as Color); 

    let action_id_s2_2 = generator_ids::get_action_id(n, 2 as Km, 2 as Color); 
    let action_id_s2_4 = generator_ids::get_action_id(n, 2 as Km, 4 as Color); 

    let action_id_s3_6 = generator_ids::get_action_id(n, 3 as Km, 6 as Color); 
    
    //## Create graph
    let graph = PathGraph::new(n, b_max, color_origin.clone(), action_id_s0_0);

    let mut graph_1 = graph.clone();
    graph_1.make_up(2 as Color, action_id_s1_2);
    graph_1.make_up(4 as Color, action_id_s2_4);

    let mut graph_2 = graph.clone();
    graph_2.make_up(4 as Color, action_id_s1_4);
    graph_2.make_up(2 as Color, action_id_s2_2);

    assert_eq!(graph_1.action_parent_id().unwrap(), action_id_s2_4);
    assert_eq!(graph_2.action_parent_id().unwrap(), action_id_s2_2);

    // No se puede fusionar porque no tienen el mismo action_parent_id
    let they_cannot_fusion = graph_1.join(&graph_2) == false;
    assert!(they_cannot_fusion);

    graph_1.make_up(6 as Color, action_id_s3_6);
    graph_2.make_up(6 as Color, action_id_s3_6);

    // Join applied
    graph_1.join(&graph_2);
    assert_eq!(graph_1.valid(),true);

    return graph_1;
}

#[test]
fn test_join(){
    let n : Color = 10 as Color;
    let b_max = 10 as Km;

    let action_id_s0_0 = generator_ids::get_action_id(n, 0 as Km, 0 as Color); 
    let action_id_s1_2 = generator_ids::get_action_id(n, 1 as Km, 2 as Color); 
    let action_id_s1_4 = generator_ids::get_action_id(n, 1 as Km, 4 as Color); 
    let action_id_s2_2 = generator_ids::get_action_id(n, 2 as Km, 2 as Color); 
    let action_id_s2_4 = generator_ids::get_action_id(n, 2 as Km, 4 as Color); 
    let action_id_s3_6 = generator_ids::get_action_id(n, 3 as Km, 6 as Color); 

    let node_id_s0_0 = NodeId::new_root(n, b_max, action_id_s0_0);
    let node_id_s1_2 = NodeId::new(n,b_max, 1 as Step,action_id_s1_2, action_id_s0_0);
    let node_id_s1_4 = NodeId::new(n,b_max, 1 as Step,action_id_s1_4, action_id_s0_0);

    let node_id_s2_4 = NodeId::new(n,b_max, 2 as Step,action_id_s2_4, action_id_s1_2);
    let node_id_s2_2 = NodeId::new(n,b_max, 2 as Step,action_id_s2_2, action_id_s1_4);
    
    let node_id_s3_6_k22 = NodeId::new(n,b_max, 3 as Step,action_id_s3_6, action_id_s2_2);
    let node_id_s3_6_k24 = NodeId::new(n,b_max, 3 as Step,action_id_s3_6, action_id_s2_4);

    //## Join
    let graph_join = build_join();

    //testing

    let edge_00_12 = check_edge(&graph_join, &node_id_s0_0, &node_id_s1_2);
    let edge_00_14 = check_edge(&graph_join, &node_id_s0_0, &node_id_s1_4);
    let edge_12_24 = check_edge(&graph_join, &node_id_s1_2, &node_id_s2_4);
    let edge_14_22 = check_edge(&graph_join, &node_id_s1_4, &node_id_s2_2);
    let edge_22_36 = check_edge(&graph_join, &node_id_s2_2, &node_id_s3_6_k22);
    let edge_24_36 = check_edge(&graph_join, &node_id_s2_4, &node_id_s3_6_k24);

    // testing nodes by color

    let set_nodes = graph_join.table_color_nodes().get(&(0 as Color)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s0_0]);
    let set_nodes = graph_join.table_color_nodes().get(&(2 as Color)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s1_2,&node_id_s2_2]);
    let set_nodes = graph_join.table_color_nodes().get(&(4 as Color)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s1_4,&node_id_s2_4]);
    let set_nodes = graph_join.table_color_nodes().get(&(6 as Color)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s3_6_k22,&node_id_s3_6_k24]);


    // testing nodes by line

    let set_nodes = graph_join.table_lines().get(&(0 as Step)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s0_0]);
    let set_nodes = graph_join.table_lines().get(&(1 as Step)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s1_2,&node_id_s1_4]);
    let set_nodes = graph_join.table_lines().get(&(2 as Step)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s2_2,&node_id_s2_4]);
    let set_nodes = graph_join.table_lines().get(&(3 as Step)).unwrap();
    check_set_nodes(&set_nodes, vec![&node_id_s3_6_k22,&node_id_s3_6_k24]);

    // Nodes
    let node_s0_0 = graph_join.table_nodes_by_action().get_node(&action_id_s0_0, &node_id_s0_0).unwrap();
    assert_eq!(node_s0_0.have_parents(), false);
    assert_eq!(node_s0_0.have_sons(), true);
    check_dict_edges_by_nodeid(node_s0_0.sons(), vec![(&node_id_s1_2,&edge_00_12), (&node_id_s1_4,&edge_00_14)]);

    let node_s1_2 = graph_join.table_nodes_by_action().get_node(&action_id_s1_2, &node_id_s1_2).unwrap();
    assert_eq!(node_s1_2.have_parents(), true);
    check_dict_edges_by_nodeid(node_s1_2.parents(), vec![(&node_id_s0_0,&edge_00_12)]);
    assert_eq!(node_s1_2.have_sons(), true);
    check_dict_edges_by_nodeid(node_s1_2.sons(), vec![(&node_id_s2_4,&edge_12_24)]);

    let node_s1_4 = graph_join.table_nodes_by_action().get_node(&action_id_s1_4, &node_id_s1_4).unwrap();
    assert_eq!(node_s1_4.have_parents(), true);
    check_dict_edges_by_nodeid(node_s1_4.parents(), vec![(&node_id_s0_0,&edge_00_14)]);
    assert_eq!(node_s1_4.have_sons(), true);
    check_dict_edges_by_nodeid(node_s1_4.sons(), vec![(&node_id_s2_2,&edge_14_22)]);

    let node_s2_4 = graph_join.table_nodes_by_action().get_node(&action_id_s2_4, &node_id_s2_4).unwrap();
    assert_eq!(node_s2_4.have_parents(), true);
    check_dict_edges_by_nodeid(node_s2_4.parents(), vec![(&node_id_s1_2,&edge_12_24)]);
    assert_eq!(node_s2_4.have_sons(), true);
    check_dict_edges_by_nodeid(node_s2_4.sons(), vec![(&node_id_s3_6_k24,&edge_24_36)]);


    let node_s2_2 = graph_join.table_nodes_by_action().get_node(&action_id_s2_2, &node_id_s2_2).unwrap();
    assert_eq!(node_s2_2.have_parents(), true);
    check_dict_edges_by_nodeid(node_s2_2.parents(), vec![(&node_id_s1_4,&edge_14_22)]);
    assert_eq!(node_s2_2.have_sons(), true);
    check_dict_edges_by_nodeid(node_s2_2.sons(), vec![(&node_id_s3_6_k22,&edge_22_36)]);


    let node_s3_6_k24 = graph_join.table_nodes_by_action().get_node(&action_id_s3_6, &node_id_s3_6_k24).unwrap();
    assert_eq!(node_s3_6_k24.have_parents(), true);
    check_dict_edges_by_nodeid(node_s3_6_k24.parents(), vec![(&node_id_s2_4,&edge_24_36)]);
    assert_eq!(node_s3_6_k24.have_sons(), false);

    let node_s3_6_k22 = graph_join.table_nodes_by_action().get_node(&action_id_s3_6, &node_id_s3_6_k22).unwrap();
    assert_eq!(node_s3_6_k22.have_parents(), true);
    check_dict_edges_by_nodeid(node_s3_6_k22.parents(), vec![(&node_id_s2_2,&edge_22_36)]);
    assert_eq!(node_s3_6_k22.have_sons(), false);


    // TEST OWNERS
    let owners_to_check = graph_join.owners_graph();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_2, &node_id_s1_4]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_2, &node_id_s2_4]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k22, &node_id_s3_6_k24]);

    let owners_to_check = node_s0_0.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_2, &node_id_s1_4]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_2, &node_id_s2_4]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k22, &node_id_s3_6_k24]);


    let owners_to_check = node_s1_2.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_2]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_4]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k24]);

    let owners_to_check = node_s1_4.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_4]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_2]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k22]);

    let owners_to_check = node_s2_4.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_2]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_4]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k24]);

    let owners_to_check = node_s2_2.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_4]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_2]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k22]);

    let owners_to_check = node_s3_6_k24.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_2]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_4]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k24]);

    let owners_to_check = node_s3_6_k22.owners();
    check_have_owners(owners_to_check, 0 as Step, vec![&node_id_s0_0]);
    check_have_owners(owners_to_check, 1 as Step, vec![&node_id_s1_4]);
    check_have_owners(owners_to_check, 2 as Step, vec![&node_id_s2_2]);
    check_have_owners(owners_to_check, 3 as Step, vec![&node_id_s3_6_k22]);

}



#[test]
fn test_up_invalid_by_repeted_color(){
    let n : Color = 10 as Color;

    let graph_join_original = build_join();

    let mut graph_join = graph_join_original.clone();
    let action_id_s4_2 = generator_ids::get_action_id(n, 4 as Km, 2 as Color);
    graph_join.up(2 as Color, action_id_s4_2);
    assert_eq!(graph_join.valid(), false);

    let mut graph_join = graph_join_original.clone();
    let action_id_s4_4 = generator_ids::get_action_id(n, 4 as Km, 4 as Color);
    graph_join.up(4 as Color, action_id_s4_4);
    assert_eq!(graph_join.valid(), false);

}

