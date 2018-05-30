extern crate actix_web;
use actix_web::{fs, server, App, HttpRequest, HttpResponse, Responder};
use std::io;
use std::path::PathBuf;

fn handle_directory<'a, 'b>(
    dir: &'a fs::Directory,
    req: &'b HttpRequest,
) -> io::Result<HttpResponse> {
    let mut path = PathBuf::from(&dir.base);
    path.push(&dir.path);
    path.push("index.html");
    // According to the signatures at either end:
    //   https://github.com/actix/actix-web/blob/v0.6.10/src/fs.rs#L212
    //   https://github.com/actix/actix-web/blob/v0.6.10/src/fs.rs#L605-L610
    // it appears that this should compile nicely. The call to `respond_to()`
    // should give an `io::Error` to meet the requirement of this
    // `files_listing_renderer()` callback.
    //
    // For some reason, the compiler sees the return as an
    // `actix_web::error::Error` forcing me to awkwardly map to `io::Error`
    // like:
    // ```
    // result.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))
    // ```
    fs::NamedFile::open(path).respond_to(req)
}

fn main() {
    server::new(|| {
        App::new().handler(
            r"/",
            fs::StaticFiles::new(".")
                .show_files_listing()
                .files_listing_renderer(handle_directory),
        )
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
