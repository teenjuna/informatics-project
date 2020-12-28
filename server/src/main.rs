use std::convert::Infallible;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;
use warp::http::{HeaderValue, Response, StatusCode};
use warp::hyper::header::CONTENT_TYPE;
use warp::{reply, Filter, Reply};

#[tokio::main]
async fn main() {
    let get_info = warp::path!("paintings" / u16 / "info")
        .and(warp::get())
        .and_then(get_info);
    let get_image = warp::path!("paintings" / u16 / "image")
        .and(warp::get())
        .and_then(get_image);

    let api = get_info.or(get_image);

    println!("Starting server on http://127.0.0.1:8080");
    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}

async fn get_info(id: u16) -> Result<reply::Response, Infallible> {
    println!("-> Got request for info about painting {}", id);

    let path = Path::new("data").join(format!("{}.txt", id));
    Ok(match fs::read_to_string(path) {
        Ok(info) => info.into_response(),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    })
}

async fn get_image(id: u16) -> Result<reply::Response, Infallible> {
    println!("-> Got request for image of painting {}", id);

    let path = Path::new("data").join(format!("{}.jpg", id));
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            return Ok(match err.kind() {
                std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            })
        }
    };

    let mut data: Vec<u8> = Vec::new();
    Ok(match file.read_to_end(&mut data) {
        Ok(_) => Response::builder()
            .header(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"))
            .body(data.into())
            .unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string().into())
            .unwrap(),
    })
}
