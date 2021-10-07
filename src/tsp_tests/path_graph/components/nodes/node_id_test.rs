use std::hash::{Hash, Hasher};

use crate::tsp::utils::alias::{Color, Km, Step , ActionId, UniqueNodeKey, InfoActionId};
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::generator_ids;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;

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


#[test]
fn test_node_ids_for_tsp(){
    let n = 4 as Color;
    let b_max = 10 as Km;
    let max_step = n;
    let mut set_hashes : HashSet<String> = HashSet::new();


    for km_origin in 0..b_max {
        for color_origin in 0..n {
            let action_parent_id : ActionId = generator_ids::get_action_id(n, km_origin, color_origin);

            for step in 0..max_step {

                for km_destine in km_origin..b_max {
                    for color_destine in 0..n {
                        let action_id : ActionId = generator_ids::get_action_id(n, km_destine, color_destine);

                        let node_id = NodeId::new(n, b_max, step as Step, action_id, action_parent_id);
                        let mut hasher = DefaultHasher::new();
                        node_id.hash(&mut hasher);
                        let node_id_hash = hasher.finish().to_string();

                        if set_hashes.contains(&node_id_hash) {
                            println!("Collision !!");
                            assert!(false);
                        }else{
                            set_hashes.insert(node_id_hash);
                        }

                        //println!("{:x}",hasher.finish());
                    }
                }
            }
        }
    }
}