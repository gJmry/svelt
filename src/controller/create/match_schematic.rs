use crate::controller::create::schematics::{component, store};
use crate::models::schematic::Schematic;

pub fn match_schematic(schematic_name: String, name: String) {
    match Schematic::find_by_name(&schematic_name) {
        Some(Schematic::Component) => {
            component::main(name);
        },
        Some(Schematic::Store) => {
            store::main(name);
        }
        _ => {}
    }
}