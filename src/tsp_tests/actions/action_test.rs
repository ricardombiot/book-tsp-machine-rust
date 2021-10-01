use crate::tsp::actions::action::Action;
use crate::tsp::utils::alias::{Color, Km, ActionsIdSet};

#[test]
fn new_init_test() {
    let n : Color = 10;
    let b_max : Km = 100;
    let up_origin_color : Color = 0;
    let action = Action::new_init(n, b_max, up_origin_color);

    
    assert_eq!(action.id(), 1);
    assert_eq!(action.km(), 0);
    assert_eq!(action.up_color(), up_origin_color);

    assert!(action.props_parents().is_empty())
    //assert_eq!(action.parents());
}


#[test]
fn new_up_test(){
    let n : Color = 10;
    let km : Km = 1;
    let up_color : Color = 1;

    let mut parents = ActionsIdSet::new();
    parents.insert(1);
    let parents_expected = parents.clone();

    let action = Action::new_up(n, km, up_color, parents);

    assert_eq!(action.id(), 12);
    assert_eq!(action.up_color(), up_color);
    assert!(action.props_parents().eq(&parents_expected));
}