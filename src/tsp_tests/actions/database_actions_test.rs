use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::utils::alias::{Color, Km, ActionsIdSet};
use crate::tsp::utils::generator_ids;

#[test]
fn init_database_actions() {
    let n : Color = 5;
    let b_max : Km = 100;
    let color_origin : Color = 0;

    let db = DatabaseActions::new(n, b_max,color_origin);

    let init_km : Km = 0;
    let init_action_id = generator_ids::get_action_id(n, init_km, color_origin);
    
    //println!("{}", db)
    let action_option = db.get_action(init_action_id);
    assert!(action_option.is_some());

    let action = db.get_action(init_action_id).unwrap();
    //println!("{}",action);
    assert_eq!(action.id(), init_action_id);
    assert_eq!(action.km(), init_km);
    assert_eq!(action.up_color(), color_origin);
}



#[test]
fn register_up_database_actions() {
    let n : Color = 5;
    let b_max : Km = 100;
    let color_origin : Color = 0;

    let mut db = DatabaseActions::new(n, b_max,color_origin);

    let init_km : Km = 0;
    let init_action_id = generator_ids::get_action_id(n, init_km, color_origin);


    let up_km : Km = 2;
    let up_color : Color = 1;
    let mut up_parents = ActionsIdSet::new();
    up_parents.insert(init_action_id);
    db.register_up(up_km, up_color, up_parents);


    let up_action_id = generator_ids::get_action_id(n, up_km, up_color);
    println!("{}", db);

    let action_option = db.get_action(up_action_id);
    assert!(action_option.is_some());

    let action = db.get_action(up_action_id).unwrap();
    //println!("{}",action);
    assert_eq!(action.id(), up_action_id);
    assert_eq!(action.km(), up_km);
    assert_eq!(action.up_color(), up_color);
}
