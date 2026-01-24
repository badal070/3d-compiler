pub mod entity {
    use crate::ids::EntityId;
    use crate::component::Component;

    #[derive(Debug, Clone)]
    pub struct Entity {
        pub id: EntityId,
        pub kind: EntityKind,
        pub components: Vec<Component>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EntityKind {
        Solid,
        Abstract,
        Reference,
    }

    impl Entity {
        pub fn new(id: EntityId, kind: EntityKind) -> Self {
            Self {
                id,
                kind,
                components: Vec::new(),
            }
        }

        pub fn add_component(&mut self, component: Component) {
            self.components.push(component);
        }

        pub fn get_transform(&self) -> Option<&crate::component::Transform> {
            self.components.iter().find_map(|c| {
                if let Component::Transform(t) = c {
                    Some(t)
                } else {
                    None
                }
            })
        }

        pub fn get_geometry(&self) -> Option<&crate::component::Geometry> {
            self.components.iter().find_map(|c| {
                if let Component::Geometry(g) = c {
                    Some(g)
                } else {
                    None
                }
            })
        }

        pub fn get_physical(&self) -> Option<&crate::component::Physical> {
            self.components.iter().find_map(|c| {
                if let Component::Physical(p) = c {
                    Some(p)
                } else {
                    None
                }
            })
        }
    }
}
