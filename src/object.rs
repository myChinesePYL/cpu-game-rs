pub struct VData {
    pub position: glm::Vector3<f64>,
    pub viewpoint: glm::Vector3<f64>,
    surface: [glm::Vector3<f64>; 3],
}

impl VData {
    pub fn new(position: glm::Vector3<f64>, viewpoint: glm::Vector3<f64>, surface: [glm::Vector3<f64>; 3]) -> Self {
        Self {
            position,
            viewpoint,
            surface,
        }
    }
    pub fn set_surface(&mut self, index: usize, position: glm::Vector3<f64>) {
        *self.surface.get_mut(index).unwrap() = position;
    }
    pub fn get_surface(&self, index: usize) -> &glm::Vector3<f64> {
        self.surface.get(index).unwrap()
    }
}

pub struct Object {
    pub position: glm::Vector3<f64>,
    pub rotation: glm::Vector3<f64>,
    vdata: Vec<VData>,
}

impl Object {
    pub fn new(position: glm::Vector3<f64>, rotation: glm::Vector3<f64>) -> Self {
        Self {
            position,
            rotation,
            vdata: Vec::new(),
        }
    }
    pub fn add_vertex(&mut self,v: VData) {
        self.vdata.push(v);
    }
}
