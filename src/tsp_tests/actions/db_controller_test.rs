use crate::tsp::utils::alias::{ActionId};
use crate::tsp::actions::db_controller::DBController;

#[test]
fn test_db_controller() {

    let mut controller = DBController::new();

    let action_id = 1 as ActionId;

    assert_eq!(controller.can_use_it(&action_id), false);
    controller.reserve(&action_id);
    assert_eq!(controller.couter_reserves(&action_id), 1 as u32);
    assert_eq!(controller.can_use_it(&action_id), true);
    
    controller.use_it(&action_id);
    assert_eq!(controller.couter_reserves(&action_id), 0 as u32);
    assert_eq!(controller.can_use_it(&action_id), false);
    
    assert_eq!(controller.pending_to_clean_ids(), &vec![action_id]);

}

#[test]
fn test_db_controller_masive_reserve_and_use() {

    let mut controller = DBController::new();
    let action_id = 1 as ActionId;

    assert_eq!(controller.couter_reserves(&action_id), 0 as u32);
    for _i in 0..10 {
        controller.reserve(&action_id);
    }
    assert_eq!(controller.couter_reserves(&action_id), 10 as u32);
   
    for _i in 0..10 {
        controller.use_it(&action_id);
    }
    assert_eq!(controller.couter_reserves(&action_id), 0 as u32);
    
}