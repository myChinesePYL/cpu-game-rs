use log::trace;

use super::object::Object;

#[derive(Debug)]
pub struct Scene {
    object: Object,
}

impl Scene {
    pub fn new(obj: Object) -> Self {
        trace!("Created scene\n\tobject: {:?}", obj);
        Self { object: obj }
    }
}

#[derive(Debug)]
pub struct Group {
    object: Object,
    pos: glm::Vec3,
}
