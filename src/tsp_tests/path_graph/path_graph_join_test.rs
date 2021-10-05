

use crate::tsp::pathset::components::edges::edge_id::EdgeId;
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::utils::alias::{Color, Km, Step, UniqueNodeKey};
use crate::tsp::pathset::components::nodes::node_id::{NodeId, NodesIdSet};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::pathset::components::nodes::node::dict_edgeid_by_nodeid::DictEdgeIdByNodeId;
use crate::tsp::utils::generator_ids;
use crate::tsp_tests::path_graph::path_graph_test::{should_be_only_node_id, check_edge};


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
    let color_origin = 0 as Color;

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


}

fn check_dict_edges_by_nodeid(dict : &DictEdgeIdByNodeId, list_keys : Vec<(&NodeId, &EdgeId)>){
    let mut dict = dict.clone();
    for (node_id, edge_id) in list_keys {
        assert!(dict.have(node_id));

        let value_edge_id = dict.get(node_id).unwrap();

        assert!(edge_id.eq(value_edge_id));
        dict.delete(node_id);
    }
    assert!(dict.is_empty())

}

fn check_set_nodes(set_nodes: &NodesIdSet, list_keys : Vec<&NodeId>){
    let mut set_nodes = set_nodes.clone();
    for node_id in list_keys {
        assert!(set_nodes.remove(node_id));
    }
    assert!(set_nodes.is_empty())
}




/*
function test_join()
    n = Color(10)
    b = Km(10)

    action_id_s0_0 = GeneratorIds.get_action_id(Color(n), Km(0), Color(0))
    action_id_s1_2 = GeneratorIds.get_action_id(Color(n), Km(1), Color(2))
    action_id_s1_4 = GeneratorIds.get_action_id(Color(n), Km(1), Color(4))
    action_id_s2_4 = GeneratorIds.get_action_id(Color(n), Km(2), Color(4))
    action_id_s2_2 = GeneratorIds.get_action_id(Color(n), Km(2), Color(2))
    action_id_s3_6 = GeneratorIds.get_action_id(Color(n), Km(3), Color(6))

    node00_id = NodeIdentity.new(n, b, Step(0), action_id_s0_0)
    node12_id = NodeIdentity.new(n, b, Step(1),action_id_s1_2, action_id_s0_0)
    node14_id = NodeIdentity.new(n, b, Step(1), action_id_s1_4, action_id_s0_0)

    node24_id = NodeIdentity.new(n, b, Step(2),action_id_s2_4, action_id_s1_2)
    node22_id = NodeIdentity.new(n, b, Step(2),action_id_s2_2, action_id_s1_4)

    node36_22_id = NodeIdentity.new(n, b, Step(3),action_id_s3_6, action_id_s2_2)
    node36_24_id = NodeIdentity.new(n, b, Step(3),action_id_s3_6, action_id_s2_4)

    ## Join
    graph_join = build_join()

    ## Edges

    edge_00_12 = PathGraph.get_edge(graph_join, node00_id, node12_id)
    @test edge_00_12.id.origin_id == node00_id
    @test edge_00_12.id.destine_id == node12_id

    edge_00_14 = PathGraph.get_edge(graph_join, node00_id, node14_id)
    @test edge_00_14.id.origin_id == node00_id
    @test edge_00_14.id.destine_id == node14_id

    edge_12_24 = PathGraph.get_edge(graph_join, node12_id, node24_id)
    @test edge_12_24.id.origin_id == node12_id
    @test edge_12_24.id.destine_id == node24_id

    edge_14_22 = PathGraph.get_edge(graph_join, node14_id, node22_id)
    @test edge_14_22.id.origin_id == node14_id
    @test edge_14_22.id.destine_id == node22_id

    edge_22_36 = PathGraph.get_edge(graph_join, node22_id, node36_22_id)
    @test edge_22_36.id.origin_id == node22_id
    @test edge_22_36.id.destine_id == node36_22_id

    edge_24_36 = PathGraph.get_edge(graph_join, node24_id, node36_24_id)
    @test edge_24_36.id.origin_id == node24_id
    @test edge_24_36.id.destine_id == node36_24_id

    # testing nodes

    @test PathGraph.get_nodes_by_color(graph_join, Color(0)) == NodesIdSet([node00_id])
    @test PathGraph.get_nodes_by_color(graph_join, Color(2)) == NodesIdSet([node12_id, node22_id])
    @test PathGraph.get_nodes_by_color(graph_join, Color(4)) == NodesIdSet([node14_id, node24_id])
    @test PathGraph.get_nodes_by_color(graph_join, Color(6)) == NodesIdSet([node36_22_id, node36_24_id])


    node00 = PathGraph.get_node(graph_join, node00_id)
    @test PathNode.have_parents(node00) == false
    @test PathNode.have_sons(node00) == true
    @test haskey(node00.sons, node12_id) == true
    @test node00.sons[node12_id] == edge_00_12.id
    @test haskey(node00.sons, node14_id) == true
    @test node00.sons[node14_id] == edge_00_14.id

    node12 = PathGraph.get_node(graph_join, node12_id)
    @test PathNode.have_parents(node12) == true
    @test haskey(node12.parents, node00_id) == true
    @test node12.parents[node00_id] == edge_00_12.id
    @test PathNode.have_sons(node12) == true
    @test haskey(node12.sons, node24_id) == true
    @test node12.sons[node24_id] == edge_12_24.id

    node14 = PathGraph.get_node(graph_join, node14_id)
    @test PathNode.have_parents(node14) == true
    @test haskey(node14.parents, node00_id) == true
    @test node14.parents[node00_id] == edge_00_14.id
    @test PathNode.have_sons(node14) == true
    @test haskey(node14.sons, node22_id) == true
    @test node14.sons[node22_id] == edge_14_22.id

    node24 = PathGraph.get_node(graph_join, node24_id)
    @test PathNode.have_parents(node24) == true
    @test haskey(node24.parents, node12_id) == true
    @test node24.parents[node12_id] == edge_12_24.id
    @test PathNode.have_sons(node24) == true
    @test haskey(node24.sons, node36_24_id) == true
    @test node24.sons[node36_24_id] == edge_24_36.id

    node22 = PathGraph.get_node(graph_join, node22_id)
    @test PathNode.have_parents(node22) == true
    @test haskey(node22.parents, node14_id) == true
    @test node22.parents[node14_id] == edge_14_22.id
    @test PathNode.have_sons(node22) == true
    @test haskey(node22.sons, node36_22_id) == true
    @test node22.sons[node36_22_id] == edge_22_36.id

    node36_22 = PathGraph.get_node(graph_join, node36_22_id)
    @test PathNode.have_parents(node36_22) == true
    @test haskey(node36_22.parents, node22_id) == true
    @test node36_22.parents[node22_id] == edge_22_36.id
    @test PathNode.have_sons(node36_22) == false

    node36_24 = PathGraph.get_node(graph_join, node36_24_id)
    @test PathNode.have_parents(node36_24) == true
    @test haskey(node36_24.parents, node24_id) == true
    @test node36_24.parents[node24_id] == edge_24_36.id
    @test PathNode.have_sons(node36_24) == false

    #= Check Owners =#
    list_all_nodes = [node00, node12, node14, node24, node22, node36_22, node36_24]
    list_all_edges = [edge_00_12, edge_00_14, edge_12_24, edge_14_22, edge_22_36, edge_24_36]
    list_nodes_12 = [node00, node12, node24, node36_24]
    list_edges_12 = [edge_00_12, edge_12_24, edge_24_36]

    list_nodes_14 = [node00, node14, node22, node36_22]
    list_edges_14 = [edge_00_12, edge_14_22, edge_22_36]
    # node00_id
    @test Owners.have(graph_join.owners, Step(0), node00_id)

    for node in list_all_nodes
        PathNode.have_owner(node, Step(0), node00_id)
    end
    #=
    for edge in list_all_edges
        PathEdge.have_owner(edge, Step(0), node00_id)
    end
    =#

    # node12_id
    @test Owners.have(graph_join.owners, Step(1), node12_id)
    for node in list_nodes_12
        PathNode.have_owner(node, Step(1), node12_id)
    end
    #=
    for edge in list_edges_12
        PathEdge.have_owner(edge, Step(1), node12_id)
    end
    =#

    for node in list_nodes_14
        !PathNode.have_owner(node, Step(1), node12_id)
    end
    #=
    for edge in list_edges_14
        !PathEdge.have_owner(edge, Step(1), node12_id)
    end
    =#

    # node24_id
    @test Owners.have(graph_join.owners, Step(2), node24_id)
    for node in list_nodes_12
        PathNode.have_owner(node, Step(2), node24_id)
    end
    #=
    for edge in list_edges_12
        PathEdge.have_owner(edge, Step(2), node24_id)
    end
    =#


    for node in list_nodes_14
        !PathNode.have_owner(node, Step(2), node24_id)
    end
    #=
    for edge in list_edges_14
        !PathEdge.have_owner(edge, Step(2), node24_id)
    end
    =#

    # node24_id
    @test Owners.have(graph_join.owners, Step(3), node36_24_id)
    for node in list_nodes_12
        PathNode.have_owner(node, Step(3), node36_24_id)
    end
    #=
    for edge in list_edges_12
        PathEdge.have_owner(edge, Step(3), node36_24_id)
    end
    =#

    for node in list_nodes_14
        !PathNode.have_owner(node, Step(3), node36_24_id)
    end
    #=
    for edge in list_edges_14
        !PathEdge.have_owner(edge, Step(3), node36_24_id)
    end
    =#

    # node14_id
    @test Owners.have(graph_join.owners, Step(1), node14_id)
    for node in list_nodes_14
        PathNode.have_owner(node, Step(1), node14_id)
    end
    #=
    for edge in list_edges_14
        PathEdge.have_owner(edge, Step(1), node14_id)
    end
    =#

    for node in list_nodes_12
        !PathNode.have_owner(node, Step(1), node14_id)
    end
    #=
    for edge in list_edges_12
        !PathEdge.have_owner(edge, Step(1), node14_id)
    end
    =#

    # node22_id
    @test Owners.have(graph_join.owners, Step(2), node22_id)
    for node in list_nodes_14
        PathNode.have_owner(node, Step(2), node22_id)
    end
    #=
    for edge in list_edges_14
        PathEdge.have_owner(edge, Step(2), node22_id)
    end
    =#

    for node in list_nodes_12
        !PathNode.have_owner(node, Step(2), node22_id)
    end
    #=
    for edge in list_edges_12
        !PathEdge.have_owner(edge, Step(2), node22_id)
    end
    =#

    # node36_22_id
    @test Owners.have(graph_join.owners, Step(3), node36_22_id)
    for node in list_nodes_14
        PathNode.have_owner(node, Step(3), node36_22_id)
    end
    #=
    for edge in list_edges_14
        PathEdge.have_owner(edge, Step(3), node36_22_id)
    end
    =#

    for node in list_nodes_12
        !PathNode.have_owner(node, Step(3), node36_22_id)
    end
    #=
    for edge in list_edges_12
        !PathEdge.have_owner(edge, Step(3), node36_22_id)
    end
    =#

end
*/