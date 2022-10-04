extern crate actix_web;
extern crate env_logger;
extern crate mime;

use mime::IMAGE_SVG;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder, Result, HttpResponse};
use std::path::Path;
use std::fs;
use actix_web::cookie::time::format_description::parse;
use actix_web::dev::Response;
use actix_web::http::header::ContentType;
use svg_diff::{DiffStep, JsonDiff, parse_svg_string, print_svg};

#[derive(Clone)]
struct AppState {
    base_svgs: Vec<String>,
    diffs: Vec<Vec<JsonDiff>>,
}

// Function for finding the svg paths
async fn find_svgs() -> Vec<String> {
    for possible_path in vec!["./svgs", "./svg_animator/svgs", "./examples/svg_animator/svgs"] {
        let path = Path::new(possible_path);
        if path.exists() && path.is_dir() {
            let mut res: Vec<String> = fs::read_dir(path).unwrap().map(
                |f| f.unwrap().path().to_str().unwrap().to_string()
            ).collect();
            res.sort();
            return res
        }
    }
    return Vec::new();
}

#[get("/")]
async fn root() -> Result<NamedFile> {
    // Serve one of the possible pathes ...
    for possible_path in vec!["./index.html", "./svg_animator/index.html", "./examples/svg_animator/index.html"] {
        if Path::new(possible_path).exists() {
            return Ok(NamedFile::open(possible_path)?);
        }
    }
    return Ok(NamedFile::open("./index.html")?);
}

#[get("/base{index}.svg")]
async fn base_svg(path: web::Path<usize>, data: web::Data<AppState>) -> HttpResponse {
    let index = path.into_inner();
    if index > data.base_svgs.len() {
        HttpResponse::NotFound().body("Index out of bound")
    } else {
        HttpResponse::Ok()
            .content_type(mime::IMAGE_SVG)
            .body(data.base_svgs.get(index).unwrap().clone())
    }
}
#[get("/diffs.json")]
async fn diffs(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(mime::APPLICATION_JSON)
        .json(&data.diffs)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set the logger
    env_logger::init();
    // Find and load the possible svg pathes
    let svg_paths = find_svgs().await;
    print!("Going to load: {:?}\n", svg_paths);
    let svgs: Vec<String> = svg_paths.iter().map(
        |p| fs::read_to_string(p).unwrap()
    ).collect();

    // Create the base and diff
    let (base_svgs, svg_diffs) = svg_diff::diff_from_strings(&svgs).unwrap();

    // Create state
    let mut diff_strings = Vec::new();
    for d in &svg_diffs {
        diff_strings.push(DiffStep::write_json(d).unwrap());
    }
    let state = AppState {
        base_svgs: base_svgs,
        diffs: diff_strings
    };

    println!("Starting http server on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(root)
            .service(base_svg)
            .service(diffs)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
