use fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, io};
use warp::http::{HeaderValue, Response, StatusCode as Code};
use warp::hyper::header::CONTENT_TYPE;
use warp::{reply, Filter, Reply};

#[tokio::main]
async fn main() {
    let get_painting_info = warp::path!("painting" / u16 / "info")
        .and(warp::get())
        .map(|id| {
            println!("-> Got request for info about painting {}", id);
            let path = Path::new("data").join(format!("{}.txt", id));
            match fs::read_to_string(path) {
                Ok(info) => reply::with_status(info, Code::OK),
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => reply::with_status(
                        format!("info for painting {} is not found", id),
                        Code::NOT_FOUND,
                    ),
                    _ => reply::with_status(
                        format!("internal server error: {}", err),
                        Code::INTERNAL_SERVER_ERROR,
                    ),
                },
            }
        });
    let get_painting_image = warp::path!("painting" / u16 / "image")
        .and(warp::get())
        .map(|id| {
            println!("-> Got request for image of painting {}", id);
            let path = Path::new("data").join(format!("{}.jpg", id));
            match File::open(path) {
                Ok(file) => Image(file).into_response(),
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => reply::with_status(
                        format!("image for painting {} is not found", id),
                        Code::NOT_FOUND,
                    ),
                    _ => reply::with_status(
                        format!("internal server error: {}", err),
                        Code::INTERNAL_SERVER_ERROR,
                    ),
                }
                .into_response(),
            }
        });

    println!("Starting server on http://127.0.0.1:8080");
    let routes = get_painting_info.or(get_painting_image);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

struct Image(File);

impl Reply for Image {
    #[inline]
    fn into_response(mut self) -> warp::reply::Response {
        let mut data: Vec<u8> = Vec::new();
        match self.0.read_to_end(&mut data) {
            Ok(_) => Response::builder()
                .header(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"))
                .body(data.into())
                .unwrap(),
            Err(err) => Response::builder()
                .status(Code::INTERNAL_SERVER_ERROR)
                .body(err.to_string().into())
                .unwrap(),
        }
    }
}
