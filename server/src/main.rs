use warp::Filter;

#[tokio::main]
async fn main() {
    let painting = warp::path!("paintings" / u16 / ..).and(warp::get());
    let info = painting
        .and(warp::path!("info"))
        .and_then(handlers::get_info);
    let image = painting
        .and(warp::path!("image"))
        .and_then(handlers::get_image);

    let api = info.or(image);

    println!("Starting server on http://127.0.0.1:8080");
    warp::serve(api).run(([0, 0, 0, 0], 8080)).await;
}

mod handlers {
    use std::convert::Infallible;
    use std::fs::{self, File};
    use std::io::{prelude::*, ErrorKind::NotFound};
    use std::path::Path;
    use warp::http::StatusCode;
    use warp::reply::Response;
    use warp::Reply;

    pub async fn get_info(id: u16) -> Result<Response, Infallible> {
        println!("-> Got request for info about painting {}", id);

        let path = Path::new("data").join(format!("{}.txt", id));
        Ok(match fs::read_to_string(path) {
            Ok(info) => info.into_response(),
            Err(err) => match err.kind() {
                NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
            .into_response(),
        })
    }

    pub async fn get_image(id: u16) -> Result<Response, Infallible> {
        use warp::http::{HeaderValue, Response};
        use warp::hyper::header::CONTENT_TYPE;

        println!("-> Got request for image of painting {}", id);

        let path = Path::new("data").join(format!("{}.jpg", id));
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Ok(match err.kind() {
                    NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                }
                .into_response())
            }
        };

        let mut data: Vec<u8> = Vec::new();
        Ok(match file.read_to_end(&mut data) {
            Ok(_) => Response::builder()
                .header(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"))
                .body(data.into())
                .unwrap(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        })
    }
}
