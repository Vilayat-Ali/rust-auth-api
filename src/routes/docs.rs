use actix_files as fs;
use actix_web::HttpRequest;
use fs::NamedFile;
use std::path::PathBuf;

pub async fn serve_docs(req: HttpRequest) -> std::io::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("index.html").parse().unwrap();
    Ok(NamedFile::open(path)?)
}
