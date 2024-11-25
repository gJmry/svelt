use crate::controller::create::schematics::*;
use crate::models::schematic::Schematic;

pub fn match_schematic(schematic_name: String, name: String) {
    match Schematic::find_by_name(&schematic_name) {
        Some(Schematic::Component) => {
            component::main(name);
        }
        Some(Schematic::Store) => {
            store::main(name);
        }
        Some(Schematic::Page) => {
            page::main(name);
        }
        Some(Schematic::Service) => {
            service::main(name);
        }
        _ => {}
    }
}
