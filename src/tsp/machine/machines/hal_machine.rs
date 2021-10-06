use crate::tsp::utils::alias::{Step, Km, Color};
use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;

use std::fmt::Debug;

#[derive(Debug)]
pub struct HamiltonianMachine {
    n : Color,
    actual_km : Km,
    color_origin : Color,
    graf : Grafo,
    timeline : Timeline,
    db : DatabaseActions
}

pub mod hal_machine_constructor;
pub mod hal_machine_send_destines;
pub mod hal_machine_execute;