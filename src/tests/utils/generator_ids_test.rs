use crate::tsp::utils::alias::{Color, Km, ActionId};
use crate::tsp::utils::generator_ids;

#[test]
fn test_create_id(){

    let n : Color = 4;
    let km : Km = 0;
    let up_color : Color = 0; 
    let expected_action_id: ActionId = 1;
    let action_id = generator_ids::get_action_id(n, km, up_color);

    assert_eq!(action_id, expected_action_id);


    let n : Color = 4;
    let km : Km = 1;
    let up_color : Color = 1; 
    let expected_action_id: ActionId = 6;
    let action_id = generator_ids::get_action_id(n, km, up_color);

    assert_eq!(action_id, expected_action_id);
}

#[test]
fn test_get_info_id(){
    
    let n : Color = 4;
    let action_id: ActionId = 1;
    let expected_km : Km = 0;
    let expected_up_color : Color = 0; 
    
    let (km, up_color) = generator_ids::get_info_id(n, action_id);

    assert_eq!(km, expected_km);
    assert_eq!(up_color, expected_up_color);


    let n : Color = 4;
    let action_id: ActionId = 6;
    let expected_km : Km = 1;
    let expected_up_color : Color = 1; 
    
    let (km, up_color) = generator_ids::get_info_id(n, action_id);

    assert_eq!(km, expected_km);
    assert_eq!(up_color, expected_up_color);
}


#[test]
fn test_matriz_ids(){
    let n = 4;
    let mut expected_action_id : ActionId = 1;
    for iter_km in 0..n  {
        let km: Km = iter_km as Km;
        for iter_color in 0..n {
            let up_color : Color = iter_color as Color;
            
            let action_id = generator_ids::get_action_id(n, km, up_color);
            //println!("KM:{}  UP_COLOR {} id: {}", km, up_color, action_id);
            assert_eq!(action_id, expected_action_id);

            let (result_km, result_up_color) = generator_ids::get_info_id(n, action_id);

            assert_eq!(result_km, km);
            assert_eq!(result_up_color, up_color);


            expected_action_id+= 1;
        }
    }
}


#[test]
fn test_matriz_ids_for_tsp(){
    let km_max : Km = 10;
    let n = 4;
    let mut expected_action_id : ActionId = 1;
    for iter_km in 0..km_max {
        let km: Km = iter_km as Km;
        for iter_color in 0..n {
            let up_color : Color = iter_color as Color;
            
            let action_id = generator_ids::get_action_id(n, km, up_color);
            //println!("KM:{}  UP_COLOR {} id: {}", km, up_color, action_id);
            assert_eq!(action_id, expected_action_id);

            let (result_km, result_up_color) = generator_ids::get_info_id(n, action_id);

            assert_eq!(result_km, km);
            assert_eq!(result_up_color, up_color);


            expected_action_id+= 1;
        }
    }
}