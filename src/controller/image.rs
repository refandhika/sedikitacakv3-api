use actix_web::{post, get, delete, web, HttpRequest, HttpResponse, Error};
use actix_files::NamedFile;
use futures_util::stream::StreamExt as _;
use actix_multipart::Multipart;
use serde::{Deserialize, Serialize};
use sanitize_filename::sanitize;

use std::fs;
use std::path::PathBuf;
use std::io::Write;
use std::fs::File;

use crate::constants::APPLICATION_JSON;

#[derive(Debug, Deserialize, Serialize)]
struct ImageRequest {
    filename: String,
}

async fn save_image(mut payload: Multipart) -> Result<ImageRequest, Error> {
    let mut saved_file: PathBuf = PathBuf::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();
        let filename = &content_disposition.expect("File not found!")
            .get_filename()
            .map(|filename| sanitize(filename))
            .unwrap_or_else(|| "upload_file".to_string());

        let mut filepath = PathBuf::from(format!("./uploads/{}", filename));

        let mut c = 1;
        while filepath.exists() {
            let file_stem = filepath
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            let extension = filepath
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            let new_filename = if extension.is_empty() {
                format!("{}-copy{}", file_stem, c)
            } else {
                format!("{}-copy{}.{}", file_stem, c, extension)
            };
            filepath = PathBuf::from(format!("./uploads/{}", new_filename));
            c += 1;
        }

        saved_file = filepath.clone();
        let mut f = File::create(filepath)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }
    }

    let saved_file_string = saved_file.to_string_lossy();

    Ok(ImageRequest {
        filename: format!("{}{}", "/assets/", saved_file_string.to_string())
    })
}

fn get_all_image() -> Result<Vec<ImageRequest>, Error> {
    let paths = fs::read_dir("./uploads").unwrap();
    let images: Vec<ImageRequest> = paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path().file_name().and_then(|n| n.to_str().map(|s| ImageRequest { filename: s.to_string() }))
            })
        })
        .collect();

    Ok(images)
}

fn get_single_image(filename: String) -> Result<ImageRequest, Error> {
    let paths = fs::read_dir("./uploads").unwrap();
    let mut found_file = "".to_string();
    for entry in paths {
        if let Ok(e) = entry {
            if let Some(file_name) = e.path().file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str == filename {
                        found_file = format!("{}{}", "/assets/", file_name_str.to_string());
                    }
                }
            }
        }
    }

    Ok(ImageRequest {
        filename: found_file.to_string(),
    })
}

// Routing

#[post("/image")]
pub async fn upload(payload: Multipart) -> HttpResponse {
    match save_image(payload).await {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Image upload success!"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Image upload failed!"})),
    }
}

#[get("/image")]
pub async fn get(img_req: web::Json<ImageRequest>) -> HttpResponse {
    let filename = &img_req.filename;

    match get_single_image(filename.to_string()) {
        Ok(image) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(image),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Image not found!"}))
    }
}

#[delete("/image")]
pub async fn delete(img_req: web::Json<ImageRequest>) -> Result<HttpResponse, Error> {
    let filepath = format!("./uploads/{}", img_req.filename);

    if fs::remove_file(filepath).is_ok() {
        Ok(HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Image deleted"})))
    } else {
        Ok(HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "File not found"})))
    }
}

#[get("/images")]
pub async fn all() -> HttpResponse {
    match get_all_image() {
        Ok(images) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(images),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to get all images"}))
    }
}

#[get("/assets/{filename:.*}")]
pub async fn serve(request: HttpRequest) -> Result<NamedFile, Error> {
    let filename: String = request.match_info().query("filename").parse().unwrap();
    println!("{}", filename);
    let filepath: PathBuf = PathBuf::from(format!("./uploads/{}", filename));

    Ok(NamedFile::open(filepath)?)
}
