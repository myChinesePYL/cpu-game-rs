use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    id: u32,
    name: u32,
    pub position: glm::Vec3,
    pub viewpoint: glm::Vec3,
    pub surface: [Vertex; 3],
}

/// 实现了`ObjData`的都是`Object`的数据
pub trait ObjData {}

/// `LData`直线数据，由2个顶点`Vertex`组成
#[derive(Clone, Debug)]
pub struct LData {
    id: u32,
    name: u32,
    pub position: glm::Vec3,
    pub viewpoint: glm::Vec3,
    pub line: [Vertex; 2],
}

impl ObjData for DData {}
impl DData {
    pub fn new(
        id: u32,
        name: u32,
        position: glm::Vec3,
        viewpoint: glm::Vec3,
        surface: [Vertex; 3],
    ) -> Self {
        Self {
            id,
            name,
            position,
            viewpoint,
            surface,
        }
    }
}
impl PartialEq for DData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
impl Hash for DData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}
impl Eq for DData {}

impl ObjData for LData {}
impl LData {
    pub fn new(
        id: u32,
        name: u32,
        position: glm::Vec3,
        viewpoint: glm::Vec3,
        line: [Vertex; 2],
    ) -> Self {
        Self {
            id,
            name,
            position,
            viewpoint,
            line,
        }
    }
}
impl PartialEq for LData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
impl Hash for LData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}
impl Eq for LData {}

/// `Object`一个对象，由若干个三角形数据`DData`和若干个直线数据`LData`组成
pub struct Object {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    vdata: HashMap<DData, DData>,
    ldata: HashMap<LData, LData>,
}

impl Object {
    pub fn new(position: glm::Vec3, rotation: glm::Vec3) -> Self {
        Self {
            position,
            rotation,
            vdata: HashMap::new(),
            ldata: HashMap::new(),
        }
    }
    pub fn add_vertex(&mut self, v: DData) {
        self.vdata.insert(v.clone(), v.clone());
    }
    /// 传入添加时的数据`v`
    pub fn remove_vertex(&mut self, v: DData) {
        //
    }
    pub fn add_line(&mut self, l: LData) {
        self.ldata.insert(l.clone(), l.clone());
    }
    pub fn remove_line(&mut self, l: LData) {
        //
    }
}
