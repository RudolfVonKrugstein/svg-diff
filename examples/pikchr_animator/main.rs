extern crate actix_web;
extern crate env_logger;
extern crate mime;
extern crate pikchr;
extern crate serde;
extern crate serde_json;


use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpServer, Result, HttpResponse};
use std::path::Path;


use std::sync::Mutex;



use actix_web::middleware::Logger;
use actix_web::web::Bytes;
use log::{info};
use pikchr::{PikchrFlags};
use serde::Serialize;
use svg_diff::{diff_from_strings, DiffStep};

struct AppState {
    last_svg: Mutex<String>
}

#[get("/")]
async fn root() -> Result<NamedFile> {
    // Serve one of the possible pathes ...
    for possible_path in vec!["./index.html", "./pikchr_animator/index.html", "./examples/pikchr_animator/index.html"] {
        if Path::new(possible_path).exists() {
            return Ok(NamedFile::open(possible_path)?);
        }
    }
    return Ok(NamedFile::open("./index.html")?);
}

#[get("/js/animator.js")]
async fn animator_js() -> Result<NamedFile> {
    // Serve one of the possible pathes ...
    for possible_path in vec!["./js/animator.js", "./pikchr_animator/js/animator.js", "./examples/pikchr_animator/js/animator.js"] {
        if Path::new(possible_path).exists() {
            return Ok(NamedFile::open(possible_path)?);
        }
    }
    return Ok(NamedFile::open("./js/animator.js")?);
}

#[derive(Serialize)]
struct ResultObject {
    svg: String,
    diffs: Vec<DiffStep>,
}

#[get("/svg")]
async fn get_svg(data: web::Data<AppState>) -> HttpResponse {
    let old_svg = data.last_svg.lock().unwrap();
    HttpResponse::Ok()
        .content_type(mime::IMAGE_SVG)
        .body((*old_svg).clone())
}

#[post("/new_diagram")]
async fn new_diagram(payload: Bytes, data: web::Data<AppState>) -> HttpResponse {
    // Convert the payload to svg
    let input = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let res = pikchr::Pikchr::render(input.as_str(), None, PikchrFlags::default());
    let svg = match res {
        Ok(p) => String::from_utf8(p.bytes().collect()).unwrap(),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    if svg.contains("<!-- empty pikchr diagram -->") {
        info!("empty diagram");
        return HttpResponse::Ok().json("{}")
    }

    // Old svg
    let mut old_svg = data.last_svg.lock().unwrap();
    if (*old_svg).len() == 0 {
        *old_svg = svg;
        return HttpResponse::Ok().json("{}")
    }
    let start_svg = (*old_svg).clone();
    *old_svg = svg.clone();

    // Retrieve the old svg
    let (new_svgs, diffs) = {
        match diff_from_strings(&vec![start_svg, svg]) {
            Ok(r) => r,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    };

    HttpResponse::Ok()
        .json(ResultObject {
            svg: new_svgs[0].clone(),
            diffs: diffs[0].clone(),
        })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set the logger
    env_logger::init();

    // Create the base and diff
    // let (base_svgs, svg_diffs) = svg_diff::diff_from_strings(&svgs).unwrap();

    // Create state
    // let mut diff_strings = Vec::new();
    // for d in &svg_diffs {
    //     diff_strings.push(DiffStep::write_json(d).unwrap());
    // }
    let state = web::Data::new(AppState {
        last_svg: Mutex::new("<svg viewBox=\"0 0 1 1\"></svg>".to_string()),
    });

    println!("Starting http server on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(root)
            .service(get_svg)
            .service(animator_js)
            .service(new_diagram)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
