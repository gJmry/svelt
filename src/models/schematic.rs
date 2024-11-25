#[derive(Debug)]
pub enum Schematic {
    Component,
    Store,
    Page,
    Service,
}
impl Schematic {
    pub fn all() -> Vec<Schematic> {
        vec![
            Schematic::Component,
            Schematic::Store,
            Schematic::Page,
            Schematic::Service,
        ]
    }

    pub fn command(&self) -> &str {
        match self {
            Schematic::Component => "component",
            Schematic::Store => "store",
            Schematic::Page => "page",
            Schematic::Service => "service",
        }
    }
    pub fn label(&self) -> &str {
        match self {
            Schematic::Component => "Component",
            Schematic::Store => "Store",
            Schematic::Page => "Page",
            Schematic::Service => "Service",
        }
    }
    pub fn hint(&self) -> &str {
        match self {
            Schematic::Component => "Generate a new Svelte component.",
            Schematic::Store => "Generate a new Svelte store.",
            Schematic::Page => "Generate a new Svelte page (for routing).",
            Schematic::Service => "Generate a new service (API or business logic).",
        }
    }

    pub fn find_by_name(name: &str) -> Option<Schematic> {
        Schematic::all()
            .into_iter()
            .find(|schematic| schematic.command() == name)
    }
}
