use actix_web::{get, post, web,Result, App, HttpResponse, HttpRequest,HttpServer, Responder};
use serde::Serialize;
use rusqlite::Connection;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

mod  data_structure;
mod data_base;

use data_structure::MemberInfo;
use data_base::query_data;

// 几个 api
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// get获取 id
#[get("/api/users/{id}")] // <- define path parameters
async fn member(req: HttpRequest) -> Result<String> {
    let id: i32 = req.match_info().query("id").parse().unwrap();
    Ok(format!("user_id {}!",id))
}

// 返回全部成员的头像、身份、名字
#[get("/api/icu_all")]
async fn all_member(conn: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {

    // 共享数据库连接
    let conn = conn.lock().await;

    // HttpResponse::Ok()
    //     .content_type("application/json")  // 设置为 JSON 格式
    //     .json(member_info)  // 返回 JSON 响应
    match query_data(&conn) {
        Ok(member_info) => HttpResponse::Ok().json(member_info),
        Err(err) => {
            let error_message = format!("{:?}", err);
            let response = json!({ "error": error_message });
            HttpResponse::InternalServerError().json(response)
        }
    }
}


// get 通过uid得到成员信息
#[get("/api/member/{id}")]
async fn icu_member_info() -> impl Responder {
    let member_info_use = MemberInfo {
        id: 0,
        name: String::from("柒灵子"),
        position: String::from("主催"),
        avatar: String::from("https://hifuufantasy.club/icu_avatar?id=0"),
        introduce: String::from("这个人是社团的主催，浙江省湖州人，十七岁是男娘。"),
    };
    // let member_info = query_data(conn);

    HttpResponse::Ok()
        .content_type("application/json")  // 设置为 JSON 格式
        .json(member_info_use)  // 返回 JSON 响应
}


// 测试发送中文字符
#[get("/test")]
async fn test() -> impl Responder{
    HttpResponse::Ok()                                                                                                            
    .content_type("text/plain; charset=utf-8")
    .body("怎么回事呢")
}

// 测试post
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// 测试自定义结构体
#[derive(Serialize)]
struct MyResponse {
    message: String,
    status: u16,
}

// 测试发送json
#[get("/json_test")]
async fn json_test() -> impl Responder {
    let response_data = MyResponse {
        message: "Hello, Actix!".to_string(),
        status: 200,
    };
    HttpResponse::Ok().json(response_data)  // 返回 JSON 响应
}

// 测试手动路由
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn open_db() -> Connection {
    Connection::open("mfhx_db.db").unwrap_or_else(|e| {
        eprintln!("数据库连接失败: {}", e);
        std::process::exit(1); // 程序失败退出
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 连接数据库

    let conn = open_db();

    // 使用 Arc 和 Mutex 包装数据库连接，便于在多个异步任务间共享
    let shared_conn = web::Data::new(Arc::new(Mutex::new(conn)));


    HttpServer::new(move || {
        App::new()
            .app_data(shared_conn.clone())
            .service(hello)
            .service(echo)
            .service(test)
            .service(member)
            .service(json_test) // 通过服务注册
            .service(all_member)
            .service(icu_member_info)
            .route("/hey", web::get().to(manual_hello)) // 手动路由
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
