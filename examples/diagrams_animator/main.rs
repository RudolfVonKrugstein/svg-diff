extern crate actix_web;
extern crate env_logger;
extern crate mime;
extern crate pikchr;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Result};
use std::path::Path;

use std::sync::Mutex;

use actix_web::middleware::Logger;
use actix_web::web::Bytes;
use git_graph_to_svg::options::layout::LayoutOptions;
use log::{info, warn};
use pikchr::PikchrFlags;
use serde::{Deserialize, Serialize};
use svg_diff::config::Config;
use svg_diff::{config::MatchingRule, config::MatchingRules, diff_from_strings, DiffStep};

struct AppState {
    last_svg: Mutex<String>,
}

#[get("/")]
async fn root() -> Result<NamedFile> {
    // Serve one of the possible pathes ...
    for possible_path in &[
        "./index.html",
        "./diagrams_animator/index.html",
        "./examples/diagrams_animator/index.html",
    ] {
        if Path::new(possible_path).exists() {
            return Ok(NamedFile::open(possible_path)?);
        }
    }
    Ok(NamedFile::open("./index.html")?)
}

#[get("/default_rules")]
async fn default_rules() -> Result<NamedFile> {
    // Serve one of the possible pathes ...
    for possible_path in &[
        "./default_matching_rules.yml",
        "./diagrams_animator/default_matching_rules.yml",
        "./examples/diagrams_animator/default_matching_rules.yml",
    ] {
        if Path::new(possible_path).exists() {
            return Ok(NamedFile::open(possible_path)?);
        }
    }
    Ok(NamedFile::open("./default_matching_rules.yml")?)
}

#[get("/js_assets/animator.js")]
async fn animator_js() -> Result<NamedFile> {
    // Serve one of the possible pathes ...
    for possible_path in &[
        "./examples/diagrams_animator/js_assets/animator.js",
        "../js_assets/animator.js",
        "./examples/js_assets/animator.js",
    ] {
        if Path::new(possible_path).exists() {
            return Ok(NamedFile::open(possible_path)?);
        }
    }
    Ok(NamedFile::open("./js/animator.js")?)
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

#[derive(Deserialize)]
struct NewDiagramPayload {
    diagram: String,
    rules: String,
    priorities: Vec<String>,
}

#[post("/new_diagram/pikchr")]
async fn new_pikchr_diagram(
    payload: web::Json<NewDiagramPayload>,
    data: web::Data<AppState>,
) -> HttpResponse {
    // Parse the rules
    let rules: Vec<MatchingRule> = match serde_yaml::from_str(&payload.rules) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    // Convert the payload to svg
    let res = pikchr::Pikchr::render(&payload.diagram, None, PikchrFlags::default());
    let svg = match res {
        Ok(p) => String::from_utf8(p.bytes().collect()).unwrap(),
        Err(e) => return HttpResponse::BadRequest().body(e),
    };
    if svg.contains("<!-- empty pikchr diagram -->") {
        info!("empty diagram");
        return HttpResponse::Ok().json("{}");
    }

    // Old svg
    let mut old_svg = data.last_svg.lock().unwrap();
    if (*old_svg).is_empty() {
        *old_svg = svg;
        return HttpResponse::Ok().json("{}");
    }
    let start_svg = (*old_svg).clone();
    *old_svg = svg.clone();

    // Retrieve the old svg
    let (new_svgs, diffs) = {
        match diff_from_strings(
            &[start_svg, svg],
            &Config {
                matching: MatchingRules {
                    rules,
                    priorities: payload.priorities.clone(),
                },
            },
        ) {
            Ok(r) => r,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    };

    HttpResponse::Ok().json(ResultObject {
        svg: new_svgs[0].clone(),
        diffs: diffs[0].clone(),
    })
}

#[post("/new_diagram/git")]
async fn new_git_diagram(
    payload: web::Json<NewDiagramPayload>,
    data: web::Data<AppState>,
) -> HttpResponse {
    // Parse the rules
    let rules: Vec<MatchingRule> = match serde_yaml::from_str(&payload.rules) {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    // Convert the payload to svg
    let res = git_graph_to_svg::parse_git_instructions(&payload.diagram);
    let res = match res {
        Ok(m) => git_graph_to_svg::print_pikchr(
            &git_graph_to_svg::model::View::from_state(&m),
            &LayoutOptions::default(),
        ),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let res = match res {
        Ok(s) => pikchr::Pikchr::render(s.as_str(), None, PikchrFlags::default()),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    let svg = match res {
        Ok(p) => String::from_utf8(p.bytes().collect()).unwrap(),
        Err(e) => return HttpResponse::BadRequest().body(e),
    };
    if svg.contains("<!-- empty pikchr diagram -->") {
        info!("empty diagram");
        return HttpResponse::Ok().json("{}");
    }

    // Old svg
    let mut old_svg = data.last_svg.lock().unwrap();
    if (*old_svg).is_empty() {
        *old_svg = svg;
        return HttpResponse::Ok().json("{}");
    }
    let start_svg = (*old_svg).clone();
    *old_svg = svg.clone();

    // Retrieve the old svg
    let (new_svgs, diffs) = {
        match diff_from_strings(
            &[start_svg, svg],
            &Config {
                matching: MatchingRules {
                    rules,
                    priorities: payload.priorities.clone(),
                },
            },
        ) {
            Ok(r) => r,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    };

    HttpResponse::Ok().json(ResultObject {
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
            .service(new_pikchr_diagram)
            .service(new_git_diagram)
            .service(default_rules)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
