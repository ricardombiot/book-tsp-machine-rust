use crate::tsp::utils::alias::{Color, Km, ActionId};

pub fn get_action_id(n : Color, km : Km, up_color : Color) -> ActionId {
    let row = km * n as u32;
    let coll : u32 = ((up_color % n) + 1) as u32;
    let action_id : ActionId = row + coll;
    return action_id;
}
 
pub fn get_info_id(n : Color, id : ActionId) -> (Km, Color) {
    let mut km : Km = 0;
    if id != 0 {
        let round_km = ((id-1) as f32 / (n as f32)) as f32;
        km = round_km.floor() as Km;
    }
    let color: Color = ((id-1) % (n as u32)) as usize;

    return (km, color);
}
