use crate::tsp::machine::machines::hal_machine::HamiltonianMachine;
use crate::tsp::machine::machines::hal_machine::Grafo;
use crate::tsp::utils::alias::{Color, Km};
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;

impl HamiltonianMachine {
    pub fn new(graf : Grafo, color_origin : Color) -> Self {
        let n = graf.get_n();
        let b_max = n as Km;
        let actual_km = 0 as Km;
        let timeline = Timeline::new();

        let db = DatabaseActions::new(n, b_max, color_origin);
        let mut machine = Self {n, actual_km, color_origin, graf, timeline, db};

        machine._init();

        return machine;
    }

    fn _init(&mut self){
        self.timeline.put_init(&self.n, &self.color_origin);
        let color_origin = self.color_origin;
        self.send_destines(&color_origin);
        self.actual_km+= 1 as Km;
    }
}
