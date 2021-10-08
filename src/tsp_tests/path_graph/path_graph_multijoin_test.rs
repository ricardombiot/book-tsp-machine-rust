use crate::tsp::utils::alias::{Color, Km, Step};
use crate::tsp::pathset::components::nodes::node_id::{NodeId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::utils::generator_ids;
use crate::tsp_tests::path_graph::test_utils::{check_set_nodes, check_dict_edges_by_nodeid, check_have_owners};


#[test]
pub fn test_multijoin(){
    let graph_join = build_multijoin();

    //println!("{:#?}", graph_join);
}


/*
#=
    0
  2   4    8
  4   2    5
  6   6    6
=#
*/
fn build_multijoin() -> PathGraph {
    let n : Color = 10 as Color;
    let b_max = 10 as Km;
    let color_origin = 0 as Color;

    let action_id_s0_0 = generator_ids::get_action_id(n, 0 as Km, 0 as Color); 

    let action_id_s1_2 = generator_ids::get_action_id(n, 1 as Km, 2 as Color); 
    let action_id_s1_4 = generator_ids::get_action_id(n, 1 as Km, 4 as Color); 
    let action_id_s1_8 = generator_ids::get_action_id(n, 1 as Km, 8 as Color); 

    let action_id_s2_2 = generator_ids::get_action_id(n, 2 as Km, 2 as Color); 
    let action_id_s2_4 = generator_ids::get_action_id(n, 2 as Km, 4 as Color); 
    let action_id_s2_5 = generator_ids::get_action_id(n, 2 as Km, 5 as Color); 

    let action_id_s3_6 = generator_ids::get_action_id(n, 3 as Km, 6 as Color); 
    
    //## Create graph
    let graph = PathGraph::new(n, b_max, color_origin.clone(), action_id_s0_0);

    let mut graph_1 = graph.clone();
    graph_1.make_up(2 as Color, action_id_s1_2);
    graph_1.make_up(4 as Color, action_id_s2_4);
    graph_1.make_up(6 as Color, action_id_s3_6);

    let mut graph_2 = graph.clone();
    graph_2.make_up(4 as Color, action_id_s1_4);
    graph_2.make_up(2 as Color, action_id_s2_2);
    graph_2.make_up(6 as Color, action_id_s3_6);

    let mut graph_3 = graph.clone();
    graph_3.make_up(8 as Color, action_id_s1_8);
    graph_3.make_up(5 as Color, action_id_s2_5);
    graph_3.make_up(6 as Color, action_id_s3_6);


    let graph_join = PathGraph::reduce_join(vec![&graph_1, &graph_2, &graph_3]).unwrap();
    assert!(graph_join.valid());

    return graph_join;
}



/*
#=
    0
  2   4    8
  4   2    5
  6   6    6
=#
function build_multi_join()
    # Building graphs and join
    n = Color(10)
    b = Km(10)
    action_id_s0_0 = GeneratorIds.get_action_id(Color(n), Km(0), Color(0))
    action_id_s1_2 = GeneratorIds.get_action_id(Color(n), Km(1), Color(2))
    action_id_s1_4 = GeneratorIds.get_action_id(Color(n), Km(1), Color(4))
    action_id_s1_8 = GeneratorIds.get_action_id(Color(n), Km(1), Color(8))

    action_id_s2_4 = GeneratorIds.get_action_id(Color(n), Km(2), Color(4))
    action_id_s2_2 = GeneratorIds.get_action_id(Color(n), Km(2), Color(2))
    action_id_s2_5 = GeneratorIds.get_action_id(Color(n), Km(2), Color(5))

    action_id_s3_6 = GeneratorIds.get_action_id(Color(n), Km(3), Color(6))
    ## Create graph
    graph = PathGraph.new(n, b, Color(0), action_id_s0_0)

    graph_2 = deepcopy(graph)
    PathGraph.up!(graph_2, Color(2), action_id_s1_2)
    PathGraph.up!(graph_2, Color(4), action_id_s2_4)

    graph_4 = deepcopy(graph)
    PathGraph.up!(graph_4, Color(4), action_id_s1_4)
    PathGraph.up!(graph_4, Color(2), action_id_s2_2)

    graph_8 = deepcopy(graph)
    PathGraph.up!(graph_8, Color(8), action_id_s1_8)
    PathGraph.up!(graph_8, Color(5), action_id_s2_5)

    PathGraph.up!(graph_2, Color(6), action_id_s3_6)
    PathGraph.up!(graph_4, Color(6), action_id_s3_6)
    PathGraph.up!(graph_8, Color(6), action_id_s3_6)


    graph_join = deepcopy(graph_2)
    PathGraph.join!(graph_join, graph_4)
    PathGraph.join!(graph_join, graph_8)

    return graph_join
end


function test_remove_partial()
    n = Color(10)
    b = Km(10)
    action_id_s4_2 = GeneratorIds.get_action_id(Color(n), Km(4), Color(2))
    graph_join = build_multi_join()
    PathGraph.up!(graph_join, Color(2), action_id_s4_2)
    @test graph_join.valid == true
end

test_remove_partial()

*/