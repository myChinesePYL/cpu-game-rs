use std::collections::HashMap;

/// `UV`UV数据
#[derive(Clone, Debug)]
pub struct UV {
    pub u: f32,
    pub v: f32,
}

/// `Vertex`顶点
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub color: palette::LinSrgba<f64>,
    pub uv: UV,
}
/// `DData` 三角形数据，由3个顶点`Vertex`组成
#[derive(Debug, Clone)]
pub struct DData {
    pub id: String,
    pub position: glm::Vec3,
    pub viewpoint: glm::Vec3,
    pub scale: glm::Vec3,
    pub surface: [Vertex; 3],
}

/// 实现了`ObjData`的都是`Object`的数据
pub trait ObjData {}

/// `LData`直线数据，由2个顶点`Vertex`组成
#[derive(Clone, Debug)]
pub struct LData {
    pub id: String,
    pub position: glm::Vec3,
    pub viewpoint: glm::Vec3,
    pub scale: glm::Vec3,
    pub line: [Vertex; 2],
}

impl ObjData for DData {}
impl DData {
    pub fn new(
        id: &str,
        position: glm::Vec3,
        viewpoint: glm::Vec3,
        scale: glm::Vec3,
        surface: [Vertex; 3],
    ) -> Self {
        Self {
            id: id.to_string(),
            position,
            viewpoint,
            scale,
            surface,
        }
    }
}

impl ObjData for LData {}
impl LData {
    pub fn new(
        id: &str,
        position: glm::Vec3,
        viewpoint: glm::Vec3,
        scale: glm::Vec3,
        line: [Vertex; 2],
    ) -> Self {
        Self {
            id: id.to_string(),
            position,
            viewpoint,
            scale,
            line,
        }
    }
}

/// `Object`一个对象，由若干个三角形数据`DData`和若干个直线数据`LData`组成
#[derive(Debug)]
pub struct Object {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    vdata: HashMap<String, DData>,
    ldata: HashMap<String, LData>,
}

impl Object {
    pub fn new(position: glm::Vec3, rotation: glm::Vec3, scale: glm::Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
            vdata: HashMap::new(),
            ldata: HashMap::new(),
        }
    }
    pub fn add_vertex(&mut self, val: DData) {
        self.vdata.insert(val.id.clone(), val.clone());
    }
    /// 传入添加时的数据`val`
    pub fn remove_vertex(&mut self, val: DData) {
        self.vdata.remove(&val.id);
    }
    pub fn add_line(&mut self, val: LData) {
        self.ldata.insert(val.id.clone(), val.clone());
    }
    /// 传入添加时的数据`l`
    pub fn remove_line(&mut self, id: &str) {
        self.ldata.remove(id);
    }

    /// 获取`vdata`
    pub fn get_vdata(&self) -> &HashMap<String, DData> {
        &self.vdata
    }
    /// 获取`ldata`
    pub fn get_ldata(&self) -> &HashMap<String, LData> {
        &self.ldata
    }
}
