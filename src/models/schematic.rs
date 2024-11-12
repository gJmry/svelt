use crate::models::schematic;

#[derive(Debug)]
pub enum Schematic {
    Component,
    Store,
    Page,
    Layout,
    Module,
    Service,
    Action,
    Directive,
    Test,
}
impl Schematic {

    pub fn all() -> Vec<Schematic> {
        vec![
            Schematic::Component,
            Schematic::Store,
            Schematic::Page,
            Schematic::Layout,
            Schematic::Module,
            Schematic::Service,
            Schematic::Action,
            Schematic::Directive,
            Schematic::Test,
        ]
    }

    pub fn command(&self) -> &str {
        match self {
            Schematic::Component => "component",
            Schematic::Store => "store",
            Schematic::Page => "page",
            Schematic::Layout => "layout",
            Schematic::Module => "module",
            Schematic::Service => "service",
            Schematic::Action => "action",
            Schematic::Directive => "directive",
            Schematic::Test => "test",
        }
    }
    pub fn label(&self) -> &str {
        match self {
            Schematic::Component => "Component",
            Schematic::Store => "Store",
            Schematic::Page => "Page",
            Schematic::Layout => "Layout",
            Schematic::Module => "Module",
            Schematic::Service => "Service",
            Schematic::Action => "Action",
            Schematic::Directive => "Directive",
            Schematic::Test => "Test",
        }
    }
    pub fn hint(&self) -> &str {
        match self {
            Schematic::Component => "Generate a new Svelte component.",
            Schematic::Store => "Generate a new Svelte store.",
            Schematic::Page => "Generate a new Svelte page (for routing).",
            Schematic::Layout => "Generate a new layout component.",
            Schematic::Module => "Generate a new module with components and logic.",
            Schematic::Service => "Generate a new service (API or business logic).",
            Schematic::Action => "Generate a new custom action in Svelte.",
            Schematic::Directive => "Generate a new directive in Svelte.",
            Schematic::Test => "Generate a test file for a Svelte component.",
        }
    }

    pub fn find_by_name(name: &str) -> Option<Schematic> {
        Schematic::all()
            .into_iter()
            .find(|schematic| schematic.command() == name)
    }
}