use crate::tsp::utils::alias::{Color, Km, Step , ActionId, UniqueNodeKey};
use crate::tsp::utils::generator_ids;
use crate::tsp::utils::generator_node_key;
use std::collections::HashSet;

#[test]
fn test_hamiltonian_colisions_keys(){
    let n : Color = 4;
    let b_max : Km = 4;
    test_not_colisions_keys(n, b_max);
}


fn test_not_colisions_keys(n : Color, b_max : Km){
    let max_step = n as Step;

    let mut set_keys : HashSet<UniqueNodeKey> = HashSet::new();
    let mut total_keys = 0;
    let max_keys_expected = b_max.pow(2)*(n as u32).pow(3);
    let mut max_key : UniqueNodeKey = 0 ;
    let mut min_key : UniqueNodeKey = 0 ;
    let mut flag_first_key = true;

    let mut colisions = false;

    
    for km_origin in 0..b_max {
        for color_origin in 0..n {
            let action_parent_id : ActionId = generator_ids::get_action_id(n, km_origin, color_origin);

            for step in 0..max_step {

                for km_destine in km_origin..b_max {
                    for color_destine in 0..n {
                        let action_id : ActionId = generator_ids::get_action_id(n, km_destine, color_destine);

                        let key = generator_node_key::calc_unique_node_key(n, b_max, step, action_id, action_parent_id);
                        
                        //println!("Step: {} :: ({},{}) - ({},{}) key: {}", step ,km_origin, color_origin, km_destine, color_destine, key);
                        if set_keys.contains(&key){  
                            println!("Collision");
                            assert!(false);
                            colisions = true;                            
                            //panic!("Collision!!");
                        }else{
                            assert!(true);
                            set_keys.insert(key);
                            total_keys+= 1
                        }

                        if flag_first_key {
                            max_key = key;
                            min_key = key;
                            flag_first_key = false;
                        }


                        if key > max_key {
                            max_key = key;
                        }

                        if key < min_key {
                            min_key = key;
                        }

                    }
                }
            }
        }
    }


    let ok_total_expected_keys =  total_keys < max_keys_expected;
    let ok_max_key_expected  = max_key <= max_keys_expected;
    let ok_min_key_expected = min_key >= 1 as UniqueNodeKey;
    let ok_without_collisions = colisions == false;


    assert!(ok_total_expected_keys);
    assert!(ok_max_key_expected);
    assert!(ok_min_key_expected);
    assert!(ok_without_collisions);
}
