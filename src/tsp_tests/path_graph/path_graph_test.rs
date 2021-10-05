
use crate::tsp::pathset::components::edges::edge_id;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::utils::alias::{Color, Km, Step , ActionId};
use crate::tsp::pathset::components::nodes::node_id::{NodeId,NodesIdSet};
use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::graph::path_graph::PathGraph;

use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::utils::generator_ids;

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

fn should_be_only_node_id(set_nodes : &NodesIdSet, expected_node_id : &NodeId){
    // Only should be root node
    assert_eq!(set_nodes.len(), 1);
    let root_node_id = set_nodes.iter().next().unwrap();
    println!("{:?}", root_node_id);
    let ok_root_id = root_node_id.eq(expected_node_id);
    assert!(ok_root_id);
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


    let origin_id = &node_id_s0_0;
    let destine_id = &node_id_s1_2;
    let id_edge = EdgeId::new(origin_id, destine_id);

    let edge = graph.table_edges().get(&id_edge).unwrap();
    assert_eq!(edge.id().destine_id(), destine_id);
    assert_eq!(edge.id().origin_id(), origin_id);

    // Parents 
    let node_s0_0 = graph.table_nodes_by_action().get_node(&action_id_s0_0, &node_id_s0_0).unwrap();
    let node_s1_2 = graph.table_nodes_by_action().get_node(&action_id_s1_2, &node_id_s1_2).unwrap();


    assert_eq!(node_s0_0.have_parents(), false);
    assert_eq!(node_s0_0.have_sons(), true);
    assert_eq!(node_s0_0.sons_list(), [(node_id_s1_2.clone(),id_edge.clone())]);

    assert_eq!(node_s1_2.have_parents(), true);
    assert_eq!(node_s1_2.have_sons(), false);
    assert_eq!(node_s1_2.parents_list(), [(node_id_s0_0.clone(),id_edge.clone())]);


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


    // Nodes

    let node_s0_1 = graph.table_nodes_by_action().get_node(&action_id_s0_1, &node_id_s0_1).unwrap();
    let node_s1_2 = graph.table_nodes_by_action().get_node(&action_id_s1_2, &node_id_s1_2).unwrap();
    let node_s2_4 = graph.table_nodes_by_action().get_node(&action_id_s2_4, &node_id_s2_4).unwrap();

    assert_eq!(node_s0_1.have_parents(), false);
    assert_eq!(node_s0_1.have_sons(), true);
    assert_eq!(node_s0_1.sons_list(), [(node_id_s1_2.clone(),id_edge_1_to_2.clone())]);

    assert_eq!(node_s1_2.have_parents(), true);
    assert_eq!(node_s1_2.have_sons(), true);
    assert_eq!(node_s1_2.parents_list(), [(node_id_s0_1.clone(),id_edge_1_to_2.clone())]);
    assert_eq!(node_s1_2.sons_list(), [(node_id_s2_4.clone(),id_edge_2_to_4.clone())]);

    assert_eq!(node_s2_4.have_parents(), true);
    assert_eq!(node_s2_4.have_sons(), false);
    assert_eq!(node_s2_4.parents_list(), [(node_id_s1_2.clone(),id_edge_2_to_4.clone())]);

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
/*
function test_second_up()
    n = Color(6)
    b = Km(6)
    action_id_s0_0 = GeneratorIds.get_action_id(Color(n), Km(0), Color(0))
    action_id_s1_2 = GeneratorIds.get_action_id(Color(n), Km(1), Color(2))
    action_id_s2_4 = GeneratorIds.get_action_id(Color(n), Km(2), Color(4))
    ## Create graph
    graph = PathGraph.new(n, b, Color(0), action_id_s0_0)
    PathGraph.make_up!(graph, Color(2), action_id_s1_2)
    PathGraph.make_up!(graph, Color(4), action_id_s2_4)
    @test graph.next_step == Step(3)

    ## Testing
    node0_id = NodeIdentity.new(n, b, Step(0), action_id_s0_0)
    node2_id = NodeIdentity.new(n, b, Step(1), action_id_s1_2, action_id_s0_0)
    node4_id = NodeIdentity.new(n, b, Step(2), action_id_s2_4, action_id_s1_2)

    ## Edges

    edge_02 = PathGraph.get_edge(graph, node0_id, node2_id)
    @test edge_02.id.origin_id == node0_id
    @test edge_02.id.destine_id == node2_id

    edge_24 = PathGraph.get_edge(graph, node2_id, node4_id)
    @test edge_24.id.origin_id == node2_id
    @test edge_24.id.destine_id == node4_id

    ## Nodes

    node0 = PathGraph.get_node(graph, node0_id)
    @test PathNode.have_parents(node0) == false
    @test PathNode.have_sons(node0) == true
    @test haskey(node0.sons, node2_id) == true
    @test node0.sons[node2_id] == edge_02.id

    node2 = PathGraph.get_node(graph, node2_id)
    @test PathNode.have_parents(node2) == true
    @test haskey(node2.parents, node0_id) == true
    @test node2.parents[node0_id] == edge_02.id
    @test PathNode.have_sons(node2) == true
    @test haskey(node2.sons, node4_id) == true
    @test node2.sons[node4_id] == edge_24.id

    node4 = PathGraph.get_node(graph, node4_id)
    @test PathNode.have_parents(node4) == true
    @test haskey(node4.parents, node2_id) == true
    @test node4.parents[node2_id] == edge_24.id
    @test PathNode.have_sons(node4) == false


    ## Check Owners

    @test Owners.have(graph.owners, Step(0), node0_id)
    @test PathNode.have_owner(node0, Step(0), node0_id)
    @test PathNode.have_owner(node2, Step(0), node0_id)
    @test PathNode.have_owner(node4, Step(0), node0_id)
    #@test PathEdge.have_owner(edge_02, Step(0), node0_id)
    #@test PathEdge.have_owner(edge_24, Step(0), node0_id)

    @test Owners.have(graph.owners, Step(1), node2_id)
    @test PathNode.have_owner(node0, Step(1), node2_id)
    @test PathNode.have_owner(node2, Step(1), node2_id)
    @test PathNode.have_owner(node4, Step(1), node2_id)
    #@test PathEdge.have_owner(edge_02, Step(1), node2_id)
    #@test PathEdge.have_owner(edge_24, Step(1), node2_id)

    @test Owners.have(graph.owners, Step(2), node4_id)
    @test PathNode.have_owner(node0, Step(2), node4_id)
    @test PathNode.have_owner(node2, Step(2), node4_id)
    @test PathNode.have_owner(node4, Step(2), node4_id)
    #@test PathEdge.have_owner(edge_02, Step(2), node4_id)
    #@test PathEdge.have_owner(edge_24, Step(2), node4_id)

end


test_init_graph()
test_up_graph()
test_second_up()

*/