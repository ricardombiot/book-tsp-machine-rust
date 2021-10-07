
use crate::tsp::utils::alias::{Color, Km, Step, UniqueNodeKey };
use crate::tsp::pathset::components::nodes::node_id::{NodeId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

use crate::tsp::utils::generator_ids;
use crate::tsp_tests::path_graph::test_utils::{should_be_only_node_id};

#[test]
fn test_graph_lazy_deleting(){
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

    assert_eq!(graph.valid(), true);
    graph.save_to_delete(&node_id_s1_2);
    assert_eq!(graph.valid(), false);
    graph.save_to_delete(&node_id_s2_4);


    assert_eq!(graph.nodes_to_delete().is_empty(),false);
    let mut nodes_to_remove : Vec<UniqueNodeKey> = graph.nodes_to_delete().clone().drain().map(|a| a.key()).collect();
    nodes_to_remove.sort();
    assert_eq!(nodes_to_remove, [node_id_s1_2.key().clone(), node_id_s2_4.key().clone()]);

    // Havent efect because is not valid;
    graph.apply_node_deletes();
    let mut nodes_to_remove : Vec<UniqueNodeKey> = graph.nodes_to_delete().clone().drain().map(|a| a.key()).collect();
    nodes_to_remove.sort();
    assert_eq!(nodes_to_remove, [node_id_s1_2.key().clone(), node_id_s2_4.key().clone()]);

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

}



#[test]
fn test_lazy_delete_nodes_by_color(){
    let n : Color = 6 as Color;
    let b_max = 6 as Km;
    let color_origin = 1 as Color;

    let action_id_s0_1 = generator_ids::get_action_id(n, 0 as Km, 1 as Color); 
    let action_id_s1_2 = generator_ids::get_action_id(n, 1 as Km, 2 as Color); 
    let action_id_s2_4 = generator_ids::get_action_id(n, 2 as Km, 4 as Color); 
    let action_id_s3_2 = generator_ids::get_action_id(n, 3 as Km, 2 as Color); 
    //## Create graph
    let mut graph = PathGraph::new(n, b_max, color_origin.clone(), action_id_s0_1);
    graph.up(2 as Color, action_id_s1_2);
    graph.up(4 as Color, action_id_s2_4);

    assert_eq!(graph.next_step(), 3 as Step);
    assert_eq!(graph.get_lenght(), 3 as Step);
    assert_eq!(graph.valid(), true);

    graph.up(2 as Color, action_id_s3_2);
    assert_eq!(graph.next_step(), 3 as Step);
    assert_eq!(graph.get_lenght(), 3 as Step);
    assert_eq!(graph.valid(), false);

    //Testing
    let node_id_s0_1 = NodeId::new_root(n, b_max, action_id_s0_1);
    let node_id_s1_2 = NodeId::new(n,b_max, 1 as Step,action_id_s1_2, action_id_s0_1);
    let node_id_s2_4 = NodeId::new(n,b_max, 2 as Step,action_id_s2_4, action_id_s1_2);

    assert_eq!(graph.nodes_to_delete().is_empty(),false);
    let nodes_to_remove : Vec<NodeId> = graph.nodes_to_delete().clone().drain().collect();
    assert_eq!(nodes_to_remove, [node_id_s1_2.clone()]);

    // Havent efect because is not valid;
    graph.apply_node_deletes();
    let nodes_to_remove : Vec<NodeId> = graph.nodes_to_delete().clone().drain().collect();
    assert_eq!(nodes_to_remove, [node_id_s1_2.clone()]);

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

}