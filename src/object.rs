pub struct VData {
    position: glm::Vector3<f64>,
    viewpoint: glm::Vector3<f64>,
    surface: [glm::Vector3<f64>; 3],
}

pub struct Object {
    position: glm::Vector3<f64>,
    rotation: glm::Vector3<f64>,
    vdata: Vec<VData>,
}

impl Object {
    pub fn new(pos: glm::Vector3<f64>, rot: glm::Vector3<f64>) -> Self {
        Self { position: pos, rotation: rot, vdata: Vec::new() }
    }
}
