pub mod component {
    use crate::value::{Vector3, Scalar};

    #[derive(Debug, Clone)]
    pub enum Component {
        Transform(Transform),
        Geometry(Geometry),
        Physical(Physical),
    }

    // Euler ONLY at IR level. Quaternions are renderer-level math.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Transform {
        pub position: Vector3,
        pub rotation: Vector3,  // Euler angles in radians
        pub scale: Vector3,
    }

    impl Transform {
        pub fn new(position: Vector3, rotation: Vector3, scale: Vector3) -> Self {
            Self { position, rotation, scale }
        }

        pub fn identity() -> Self {
            Self {
                position: Vector3::zero(),
                rotation: Vector3::zero(),
                scale: Vector3::one(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Geometry {
        Primitive(Primitive),
        Procedural(ProceduralShape),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Primitive {
        Cube,
        Cylinder,
        Sphere,
    }

    // No meshes. Ever.
    #[derive(Debug, Clone)]
    pub struct ProceduralShape {
        pub name: String,
        pub parameters: Vec<Scalar>,
    }

    impl ProceduralShape {
        pub fn new(name: String, parameters: Vec<Scalar>) -> Self {
            Self { name, parameters }
        }
    }

    // Optional because not all concepts are physical.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Physical {
        pub mass: Option<f64>,
        pub rigid: bool,
    }

    impl Physical {
        pub fn new(mass: Option<f64>, rigid: bool) -> Self {
            Self { mass, rigid }
        }

        pub fn rigid_body(mass: f64) -> Self {
            Self {
                mass: Some(mass),
                rigid: true,
            }
        }

        pub fn kinematic() -> Self {
            Self {
                mass: None,
                rigid: true,
            }
        }
    }
}
