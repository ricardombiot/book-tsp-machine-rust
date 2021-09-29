use crate::tsp::utils::alias::{Color, Km, ActionId, Step, InfoActionId, UniqueNodeKey};
use crate::tsp::utils::generator_ids;

pub fn calc_unique_node_key(n : Color, b : Km, step : Step, action_id : ActionId, action_parent_id : ActionId) -> UniqueNodeKey {
    let ((km_destine, color_destine), (km_origin, color_origin)) = get_info_by_actions(n, action_id, action_parent_id);
   
    let n_u32 = n as u32;
    let color_destine_u32 = color_destine as u32;
    let color_origin_u32 = color_origin as u32;

    return (step*b.pow(2)*n_u32.pow(2)) + (km_origin*(b*n_u32.pow(2))) + (color_origin_u32*b*n_u32) + (km_destine*n_u32) + (color_destine_u32) + 1;
}

pub fn get_info_by_actions(n : Color, action_id : ActionId, action_parent_id : ActionId) -> (InfoActionId, InfoActionId) {
    let destine_info = generator_ids::get_info_id(n, action_id);
    let origin_info = generator_ids::get_info_id(n, action_parent_id);

    return (destine_info, origin_info);
}

