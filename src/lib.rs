/**
使用cpu进行计算的图形库(玩具品)
例子
```toml
[dependencies]
cgame = { version = "0.0.0", package = "cpu-game" }
```
```rust
fn main() {
    let obj = cgame::object::Object::new(glm::vec3(...), glm::vec3(...), glm::vec3(...));
    let scene = cgame::scene::Scene::new(obj);
}
```
**/

// export crate
/// 数学库nalgebra
pub extern crate glm;
/// Rust官方日志库
pub extern crate log;
/// 颜色库
pub extern crate palette;
/// 搭配日志库
pub extern crate simple_logger;

static mut INITED: bool = false;

/// `init`只执行一次
pub fn init() {
    unsafe {
        if !INITED {
            INITED = true;
            simple_logger::init().unwrap();
        }
    }
}

/// 物品对象
pub mod object;
/// 场景对象
pub mod scene;
