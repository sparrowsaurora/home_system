use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

static VIDEO_ROOT: &str = "./videos";

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    let folders = fs::read_dir(VIDEO_ROOT)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().into_string().unwrap())
        .collect::<Vec<_>>();

    context.insert("folders", &folders);
    let rendered = tmpl.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn anime_folder(folder: web::Path<String>, tmpl: web::Data<Tera>) -> impl Responder {
    let folder_path = Path::new(VIDEO_ROOT).join(&*folder);
    if !folder_path.exists() {
        return HttpResponse::NotFound().body("Folder not found");
    }

    let episodes = fs::read_dir(&folder_path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path().is_file()
                && mime_guess::from_path(e.path())
                    .first_raw()
                    .map(|m| m.starts_with("video/"))
                    .unwrap_or(false)
        })
        .map(|e| e.file_name().into_string().unwrap())
        .collect::<Vec<_>>();

    let mut context = Context::new();
    context.insert("folder", &*folder);
    context.insert("episodes", &episodes);
    let rendered = tmpl.render("episodes.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn watch(info: web::Path<(String, String)>) -> Result<NamedFile> {
    let (folder, episode) = info.into_inner();
    let video_path = Path::new(VIDEO_ROOT).join(&folder).join(&episode);

    NamedFile::open_async(video_path)
        .await
        .map_err(|e| e.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();
    println!("Starting anime streamer on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/anime/{folder}", web::get().to(anime_folder))
            .route("/watch/{folder}/{episode}", web::get().to(watch))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
