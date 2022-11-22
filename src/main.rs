use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use fs::{Files, NamedFile};
use futures_util::{TryStreamExt as _, StreamExt};
use std::io::Write;
use std::path::{self, PathBuf};
use uuid::Uuid;

async fn get_image(req: HttpRequest) -> std::io::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn create_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("{filename}");

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get/{filename:.*}", web::get().to(get_image))
            .route("/upload", web::post().to(create_image))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}