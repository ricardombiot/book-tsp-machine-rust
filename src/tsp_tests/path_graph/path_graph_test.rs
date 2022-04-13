use crate::tsp::utils::alias::{Color, Km, Step , ActionId};
use crate::tsp::pathset::components::nodes::node_id::{NodeId,NodesIdSet};
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;

use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::utils::generator_ids;
use crate::tsp_tests::path_graph::test_utils::{should_be_only_node_id};





#[test]
fn test_graph_init(){
    let n : Color = 4;
    let b_max : Km = 10;
    let action_id : ActionId = 1;
    

    let expected_node_id = NodeId::new_root(n, b_max, action_id);

    let color_origin : Color = 0;
    let step_origin : Step = 0;
    let graph = PathGraph::new(n, b_max, color_origin.clone(), action_id.clone());

    assert_eq!(graph.next_step(), 1 as Step);

    let set_nodes = graph.table_color_nodes().get(&color_origin).unwrap();
    should_be_only_node_id(set_nodes, &expected_node_id);

    let set_nodes = graph.table_lines().get(&step_origin).unwrap();
    should_be_only_node_id(set_nodes, &expected_node_id);

    let node_root = graph.table_nodes_by_action().get_node(&action_id, &expected_node_id).unwrap();
    assert!(node_root.id().eq(&expected_node_id));
    assert!(node_root.parents().is_empty());
    assert!(node_root.sons().is_empty());

    graph.owners_graph().have(&expected_node_id);
    node_root.owners().have(&expected_node_id);
}



#[test]
fn test_graph_make_up(){
    let n : Color = 10 as Color;
    let b_max = 20 as Km;
    let color_origin = 0 as Color;

    let action_id_s0_0 = generator_ids::get_action_id(n, 0 as Km, 0 as Color); 
    let action_id_s1_2 = generator_ids::get_action_id(n, 1 as Km, 2 as Color); 
    //## Create graph
    let mut graph = PathGraph::new(n, b_max, 0 as Color, action_id_s0_0);
    graph.make_up(2 as Color, action_id_s1_2);

    assert_eq!(graph.next_step(), 2 as Step);
    assert_eq!(graph.get_lenght(), 2 as Step);

    let node_id_s0_0 = NodeId::new_root(n, b_max, action_id_s0_0);
    let node_id_s1_2 = NodeId::new(n,b_max, 1 as Step,action_id_s1_2, action_id_s0_0);


    let set_nodes = graph.table_color_nodes().get(&color_origin).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s0_0);
    let set_nodes = graph.table_lines().get(&(0 as Step)).unwrap();
    //println!("{:#?}",set_nodes);
    should_be_only_node_id(set_nodes, &node_id_s0_0);


    let set_nodes = graph.table_color_nodes().get(&(2 as Color)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s1_2);
    let set_nodes = graph.table_lines().get(&(1 as Step)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s1_2);


   /* let origin_id = &node_id_s0_0;
    let destine_id = &node_id_s1_2;
    let id_edge = EdgeId::new(origin_id, destine_id);

    let edge = graph.table_edges().get(&id_edge).unwrap();
    assert_eq!(edge.id().destine_id(), destine_id);
    assert_eq!(edge.id().origin_id(), origin_id);*/

    //let id_edge = check_edge(&graph, &node_id_s0_0, &node_id_s1_2);

    // Parents 
    let node_s0_0 = graph.table_nodes_by_action().get_node(&action_id_s0_0, &node_id_s0_0).unwrap();
    let node_s1_2 = graph.table_nodes_by_action().get_node(&action_id_s1_2, &node_id_s1_2).unwrap();


    assert_eq!(node_s0_0.have_parents(), false);
    assert_eq!(node_s0_0.have_sons(), true);
    assert_eq!(node_s0_0.sons_list(), [node_id_s1_2.clone()]);

    assert_eq!(node_s1_2.have_parents(), true);
    assert_eq!(node_s1_2.have_sons(), false);
    assert_eq!(node_s1_2.parents_list(), [node_id_s0_0.clone()]);


    // Owners
    assert!(graph.owners_graph().get_step_owners(0 as Step).unwrap().have(node_id_s0_0.key()));
    assert!(graph.owners_graph().get_step_owners(1 as Step).unwrap().have(node_id_s1_2.key()));

    let owners_to_check =  node_s0_0.owners();
    assert!(owners_to_check.get_step_owners(0 as Step).unwrap().have(node_id_s0_0.key()));
    assert!(owners_to_check.get_step_owners(1 as Step).unwrap().have(node_id_s1_2.key()));

    let owners_to_check =  node_s1_2.owners();
    assert!(owners_to_check.get_step_owners(0 as Step).unwrap().have(node_id_s0_0.key()));
    assert!(owners_to_check.get_step_owners(1 as Step).unwrap().have(node_id_s1_2.key()));

}



#[test]
fn test_graph_make_second_up(){
    let n : Color = 6 as Color;
    let b_max = 6 as Km;
    let color_origin = 1 as Color;

    let action_id_s0_1 = generator_ids::get_action_id(n, 0 as Km, 1 as Color); 
    let action_id_s1_2 = generator_ids::get_action_id(n, 1 as Km, 2 as Color); 
    let action_id_s2_4 = generator_ids::get_action_id(n, 2 as Km, 4 as Color); 
    //## Create graph
    let mut graph = PathGraph::new(n, b_max, color_origin.clone(), action_id_s0_1);
    graph.make_up(2 as Color, action_id_s1_2);
    graph.make_up(4 as Color, action_id_s2_4);

    assert_eq!(graph.next_step(), 3 as Step);
    assert_eq!(graph.get_lenght(), 3 as Step);


    //Testing
    let node_id_s0_1 = NodeId::new_root(n, b_max, action_id_s0_1);
    let node_id_s1_2 = NodeId::new(n,b_max, 1 as Step,action_id_s1_2, action_id_s0_1);
    let node_id_s2_4 = NodeId::new(n,b_max, 2 as Step,action_id_s2_4, action_id_s1_2);


    let set_nodes = graph.table_color_nodes().get(&color_origin).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s0_1);
    let set_nodes = graph.table_lines().get(&(0 as Step)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s0_1);

    let set_nodes = graph.table_color_nodes().get(&(2 as Color)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s1_2);
    let set_nodes = graph.table_lines().get(&(1 as Step)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s1_2);

    let set_nodes = graph.table_color_nodes().get(&(4 as Color)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s2_4);
    let set_nodes = graph.table_lines().get(&(2 as Step)).unwrap();
    should_be_only_node_id(set_nodes, &node_id_s2_4);


    // Edges 

    /*
    let origin_id = &node_id_s0_1;
    let destine_id = &node_id_s1_2;
    let id_edge_1_to_2 = EdgeId::new(origin_id, destine_id);

    let edge_1_to_2 = graph.table_edges().get(&id_edge_1_to_2).unwrap();
    assert_eq!(edge_1_to_2.id().destine_id(), destine_id);
    assert_eq!(edge_1_to_2.id().origin_id(), origin_id);

    let origin_id = &node_id_s1_2;
    let destine_id = &node_id_s2_4;
    let id_edge_2_to_4 = EdgeId::new(origin_id, destine_id);

    let edge_2_to_4 = graph.table_edges().get(&id_edge_2_to_4).unwrap();
    assert_eq!(edge_2_to_4.id().destine_id(), destine_id);
    assert_eq!(edge_2_to_4.id().origin_id(), origin_id);
     */

    // Nodes

    let node_s0_1 = graph.table_nodes_by_action().get_node(&action_id_s0_1, &node_id_s0_1).unwrap();
    let node_s1_2 = graph.table_nodes_by_action().get_node(&action_id_s1_2, &node_id_s1_2).unwrap();
    let node_s2_4 = graph.table_nodes_by_action().get_node(&action_id_s2_4, &node_id_s2_4).unwrap();

    assert_eq!(node_s0_1.have_parents(), false);
    assert_eq!(node_s0_1.have_sons(), true);
    assert_eq!(node_s0_1.sons_list(), [node_id_s1_2.clone()]);

    assert_eq!(node_s1_2.have_parents(), true);
    assert_eq!(node_s1_2.have_sons(), true);
    assert_eq!(node_s1_2.parents_list(), [node_id_s0_1.clone()]);
    assert_eq!(node_s1_2.sons_list(), [node_id_s2_4.clone()]);

    assert_eq!(node_s2_4.have_parents(), true);
    assert_eq!(node_s2_4.have_sons(), false);
    assert_eq!(node_s2_4.parents_list(), [node_id_s1_2.clone()]);

    // Owners
    assert!(graph.owners_graph().get_step_owners(0 as Step).unwrap().have(node_id_s0_1.key()));
    assert!(graph.owners_graph().get_step_owners(1 as Step).unwrap().have(node_id_s1_2.key()));
    assert!(graph.owners_graph().get_step_owners(2 as Step).unwrap().have(node_id_s2_4.key()));

    for node_selected in [node_s0_1, node_s1_2, node_s2_4].iter() {
        let owners_to_check =  node_selected.owners();
        assert!(owners_to_check.get_step_owners(0 as Step).unwrap().have(node_id_s0_1.key()));
        assert!(owners_to_check.get_step_owners(1 as Step).unwrap().have(node_id_s1_2.key()));
        assert!(owners_to_check.get_step_owners(2 as Step).unwrap().have(node_id_s2_4.key()));
    }

}