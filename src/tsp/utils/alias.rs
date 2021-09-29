use std::collections::HashSet;

// Con la actionid podemos identificar
// (Km, Color) donde se realizo.
pub type ActionId = u32 ;
pub type InfoActionId = (Km, Color);

//El step dentro de un path
pub type  Step = u32;
// El Km
pub type Km = u32;

// El numero del nodo
pub type Color = usize ;
// Peso de las aristas
pub type Weight = u32 ;


pub type UniqueNodeKey = u32;


pub type ActionsIdSet = HashSet<ActionId>;
pub type SetSteps = HashSet<Step>;
pub type SetColors = HashSet<Color>;