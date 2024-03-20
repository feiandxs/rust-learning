#[macro_use] extern crate rocket;
use reqwest;


mod schema;
use schema::{Response1, Response2, RequestBody, Response3};

#[get("/response1")]
fn handler1() -> rocket::serde::json::Json<Response1> {
    // let data: Vec<i32> = (1..=100).map(|x| x).collect::<Vec<_>>();
    let data: Vec<i32> = (1..=100).collect();

    // 这里发生了 copy 操作,因为 &data 是不可变引用,实现了 Copy trait
    let data_ref: &Vec<i32> = &data;

    // 这里必须显式调用 clone 方法,因为 Vec<i32> 没有实现 Copy trait
    let data_clone: Vec<i32> = data.clone();

    let response = Response1 {
        // 这里发生了 clone 操作,因为 String 没有实现 Copy trait
        message: "This is response1".to_string(),
        // 这里发生了 copy 操作,因为 &Vec<i32> 实现了 Copy trait
        data: data_ref.clone(),
    };

    rocket::serde::json::Json(response)
}

#[get("/response2")]
async fn handler2() -> rocket::serde::json::Json<Response2> {
    let response = reqwest::get("https://www.baidu.com")
        .await
        .unwrap();

    let status_code = response.status().as_u16();

    if status_code == 200 {
        let body = response.text().await.unwrap();
        let title = extract_title(&body);

        let response = Response2 {
            status: true,
            result: title,
        };
        rocket::serde::json::Json(response)
    } else {
        let response = Response2 {
            status: false,
            result: format!("Failed to fetch website, status code: {}", status_code),
        };
        rocket::serde::json::Json(response)
    }
}

fn extract_title(html: &str) -> String {
    // 使用正则表达式或 HTML 解析器从 HTML 中提取标题
    // 这里为了简单起见，我们假设标题总是位于 <title>...</title> 之间
    let start = html.find("<title>").map(|i| i + 7).unwrap_or(0);
    let end = html.find("</title>").unwrap_or(html.len());
    html[start..end].to_string()
}

#[post("/process", data = "<body>")]
fn process(body: rocket::serde::json::Json<RequestBody>) -> rocket::serde::json::Json<Response3> {
    let content = body.content.clone();
    // 打印content
    println!("content: {}", content);
    let random_suffix = random_string(5);
    let result = format!("{}{}", content, random_suffix);
    let response = Response3 {
        message: "Process successful".to_string(),
        data: result,
    };
    rocket::serde::json::Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handler1, handler2, process])
}

use rand::Rng;

fn random_string(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    let mut random_string = String::with_capacity(len);
    for _ in 0..len {
        let idx = rng.gen_range(0..CHARSET.len());
        random_string.push(CHARSET[idx] as char);
    }
    random_string
}

//
// 在 Rust 中,&data 是一个不可变引用(immutable reference),而不是指针(pointer)。虽然它们有一些相似之处,但它们在概念和实现上有着明确的区别。
//
// 不可变引用(&T):
//
// 不可变引用是 Rust 中的一种借用(borrowing)机制,用于安全地共享数据而不转移所有权。
// 它提供了一个指向数据的不可变视图,可以读取数据,但不能修改数据。
// 在同一个作用域中,可以存在多个不可变引用指向同一块数据。
// 引用本身不占用任何实际的内存空间,只是一个指向数据的地址。
// 引用的生命周期由 Rust 的所有权和借用规则来管理,以确保内存安全。
// 指针(pointer):
//
// 指针是一个存储内存地址的变量,可以直接操作该地址指向的数据。
// 指针可以是可变的(mutable pointer),也可以是不可变的(immutable pointer)。
// 指针没有所有权概念,它们可以自由地指向任何数据,包括无效或已释放的内存。
// 使用指针存在内存安全风险,比如空指针、悬垂指针等。
// 指针的使用需要手动管理内存分配和释放,容易导致内存泄漏或无效内存访问。
// 在你的示例代码中,&data 是一个不可变引用,它借用了 data 这个 Vec<i32> 的所有权,但不拥有所有权。它提供了一个读取 data 的视图,但不能修改 data 的内容。与之不同,如果你有一个可变引用 &mut data,你就可以通过它来修改 data 中的元素。
//
// 不可变引用是 Rust 中安全共享数据的一种机制,它避免了悬垂指针和数据竞争等问题,同时也保证了内存安全和线程安全。与传统的指针相比,引用提供了更高级别的抽象,使编程更加安全和简单。
//
// 总的来说,&data 是一个不可变引用,而不是指针。它是 Rust 借用机制的一部分,用于安全地共享数据而不转移所有权。