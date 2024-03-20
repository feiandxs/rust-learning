// src/schema/mod.rs
mod response1;
mod response2;

pub use response1::Response1;
pub use response2::Response2;


// 在 main.rs 文件中:
//
// 我们使用 mod schema; 声明并加载 schema 模块。
// 使用 use schema::{Response1, Response2}; 从 schema 模块中导入两种响应数据格式。
// 定义了两个处理器函数 handler1 和 handler2，分别返回 Response1 和 Response2 类型的 JSON 数据。
// 在 rocket() 函数中,我们挂载了这两个处理器的路由。
// 现在,当你运行 cargo run 并访问 http://localhost:8000/response1 和 http://localhost:8000/response2 时,就会分别收到不同格式的 JSON 响应数据。
//
// 通过这种方式,我们将响应数据格式封装在 schema 模块中,每种格式定义在单独的文件中。mod.rs 文件作为模块入口点,控制了对外暴露的公共接口。在 main.rs 中,我们只需要从 schema 模块中导入所需的数据格式,并使用它们即可。
//
// 这种模块化组织代码的方式不仅增强了可读性和可维护性,还提高了代码的可组合性和可扩展性。当你需要添加新的响应数据格式时,只需在 schema 目录下创建一个新文件,并在 mod.rs 中导出它即可,而无需修改其他模块的代码。