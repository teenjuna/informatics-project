use actix_web::{web, App, HttpServer};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Running server at http://localhost:8080");
    HttpServer::new(move || {
        App::new().service(
            web::scope("/paintings/{id}")
                .service(web::resource("").route(web::get().to(handlers::get_info)))
                .service(web::resource("/image").route(web::get().to(handlers::get_image))),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

mod handlers {
    use actix_files::NamedFile;
    use actix_web::{error, web, Error as HttpError, HttpResponse, Responder};
    use server::{Error, Painting};

    pub async fn get_info(id: web::Path<u16>) -> impl Responder {
        let id = id.into_inner();
        println!("trying to get info {}", id);

        match Painting::load_info(id) {
            Ok(info) => HttpResponse::Ok().json(info),
            Err(err) => match err {
                Error::InfoNotFound(_) => error::ErrorNotFound(err).into(),
                _ => error::ErrorInternalServerError(err).into(),
            },
        }
    }

    pub async fn get_image(id: web::Path<u16>) -> Result<NamedFile, HttpError> {
        let id = id.into_inner();
        println!("trying to get image {}", id);

        match Painting::image_path(id) {
            Ok(path) => match NamedFile::open(path) {
                Ok(file) => Ok(file),
                Err(err) => Err(error::ErrorInternalServerError(err)),
            },
            Err(err) => match err {
                Error::ImageNotFound(_) => Err(error::ErrorNotFound(err)),
                _ => Err(error::ErrorInternalServerError(err)),
            },
        }
    }
}
