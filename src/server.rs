mod utils;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::{
    fs::File,
    io::{Write, Read}
};

use utils::{
    split,
    rename_services,
    rename_requires,
    rename_find_childs,
    rename_dot_variables,
    rename_new_variables,
    rename_new_tables,
};

#[get("/")]
async fn index() -> impl Responder {
    format!("Server is running, request to /optimize with a `Code` header")
}

#[get("/optimize")]
async fn optimize(req: actix_web::HttpRequest) -> impl Responder {
    if let Some(header_value) = req.headers().get("Code") {
        if let Ok(request_body) = header_value.to_str() {
            let mut lines = split(request_body, "\n");
            lines = lines.iter().map(|s| s.trim().to_string()).collect();

            lines = rename_services(&mut lines).expect("Failed renaming services");

            lines = rename_requires(&mut lines).expect("Failed renaming requires");

            lines = rename_find_childs(&mut lines).expect("Failed renaming find childs");

            lines = rename_dot_variables(&mut lines).expect("Failed renaming variables");

            lines = rename_new_variables(&mut lines).expect("Failed renaming new variables");

            lines = rename_new_tables(&mut lines).expect("Failed renaming new tables");

            let output = lines.join("\n");

            let mut output_file =
                File::create("/files/output.lua").expect("Failed creating output file");

            println!("Writing to output file");

            output_file
                .write_all(output.as_bytes())
                .expect("Failed writing to output file");

            println!("Formatting output file");

            std::process::Command::new("stylua")
                .arg("/files/output.lua")
                .output()
                .expect("Failed formatting output file");

            println!("Reading formatted output file");

            let formatted_output = std::fs::read_to_string("/files/output.lua")
                .expect("Failed reading formatted output file");

            return HttpResponse::Ok()
                .content_type("text/plain")
                .body(formatted_output);
        }
    }

    HttpResponse::BadRequest().body("Invalid input, include a `Code` header")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(optimize)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
