/// `UV`UV
pub struct UV {
    pub u: f64,
    pub v: f64,
}

/// `Vertex`顶点
pub struct Vertex {
    pub position: glm::Vec3,
    pub color: palette::LinSrgba<f64>,
    pub uv: UV,
}

/// `DData` 三角形数据，由3个顶点`Vertex`组成
pub struct DData {
    pub position: glm::Vec3,
    pub viewpoint: glm::Vec3,
    surface: [Vertex; 3],
}

impl DData {
    pub fn new(position: glm::Vec3, viewpoint: glm::Vec3, surface: [Vertex; 3]) -> Self {
        Self {
            position,
            viewpoint,
            surface,
        }
    }
    pub fn set_surface(&mut self, index: usize, position: Vertex) {
        *self.surface.get_mut(index).unwrap() = position;
    }
    pub fn get_surface(&self, index: usize) -> &Vertex {
        self.surface.get(index).unwrap()
    }
}

/// `Object`一个对象，由若干个三角形数据`DData`组成
pub struct Object {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    vdata: Vec<DData>,
}

impl Object {
    pub fn new(position: glm::Vec3, rotation: glm::Vec3) -> Self {
        Self {
            position,
            rotation,
            vdata: Vec::new(),
        }
    }
    pub fn add_vertex(&mut self, v: DData) {
        self.vdata.push(v);
    }
}
