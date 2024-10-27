/// `UV`UV
pub struct UV {
    u: f64,
    v: f64,
}

/// `Vertex`顶点
pub struct Vertex {
    pub position: glm::Vector3<f64>,
    pub color: palette::LinSrgba<f64>,
    pub uv: UV,
}

/// `DData` 三角形数据，由3个顶点`Vertex`组成
pub struct DData {
    pub position: glm::Vector3<f64>,
    pub viewpoint: glm::Vector3<f64>,
    surface: [Vertex; 3],
}

impl DData {
    pub fn new(
        position: glm::Vector3<f64>,
        viewpoint: glm::Vector3<f64>,
        surface: [Vertex; 3],
    ) -> Self {
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
    pub position: glm::Vector3<f64>,
    pub rotation: glm::Vector3<f64>,
    vdata: Vec<DData>,
}

impl Object {
    pub fn new(position: glm::Vector3<f64>, rotation: glm::Vector3<f64>) -> Self {
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

#[allow(non_snake_case)]
#[must_use]
/// `create_M(px: f64, py: f64, pz: f64, theta_x: f64, theta_y: f64, theta_z: f64, sx: f64, sy: f64, sz: f64) -> glm::Matrix4<f64>`
/// `px` `py` `pz`是平移的xyz坐标
/// `theta_x` `theta_y` `theta_z`是旋转的角度0°=0.0，360°=π
/// `sx` `sy` `sz`是缩放的xyz长度
/// 返回`mart4`齐次变换矩阵
pub fn create_M(
    px: f64,
    py: f64,
    pz: f64,
    theta_x: f64,
    theta_y: f64,
    theta_z: f64,
    sx: f64,
    sy: f64,
    sz: f64,
) -> glm::Matrix4<f64> {
    let RX = glm::matrix![1.0, 0.0, 0.0, 0.0;
                          0.0, theta_x.cos(), -theta_x.sin(), 0.0;
                          0.0, theta_x.sin(), theta_x.cos(), 0.0;
                          0.0, 0.0, 0.0, 1.0];
    let RY = glm::matrix![theta_y.cos(), 0.0, theta_y.sin(), 0.0;
                          0.0, 1.0, 0.0, 0.0;
                          -theta_y.sin(), 0.0, theta_y.cos(), 0.0;
                          0.0, 0.0, 0.0, 1.0];
    let RZ = glm::matrix![theta_z.cos(), 0.0, -theta_z.sin(), 0.0;
                          theta_z.sin(), 1.0, theta_z.cos(), 0.0;
                          0.0, 0.0, 1.0, 0.0;
                          0.0, 0.0, 0.0, 1.0];
    let S = glm::matrix![sx, 0.0, 0.0, 0.0;
                         0.0, sy, 0.0, 0.0;
                         0.0, 0.0, sz, 0.0;
                         0.0, 0.0, 0.0, 1.0];
    let T = glm::matrix![1.0, 0.0, 0.0, px;
                         0.0, 1.0, 0.0, py;
                         0.0, 0.0, 1.0, pz;
                         0.0, 0.0, 0.0, 1.0];

    RZ * RY * RX * S * T
}
