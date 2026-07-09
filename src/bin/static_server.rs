use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

const ROOT: &str = "dist";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Serving {ROOT} at http://127.0.0.1:8080");

    for stream in listener.incoming().flatten() {
        let _ = handle(stream);
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 2048];
    let read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..read]);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    let file_path = resolve_path(path);
    let (status, body, content_type) = match fs::read(&file_path) {
        Ok(bytes) => ("200 OK", bytes, mime_for(&file_path)),
        Err(_) => (
            "404 Not Found",
            b"Not found".to_vec(),
            "text/plain; charset=utf-8",
        ),
    };

    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(&body)
}

fn resolve_path(request_path: &str) -> PathBuf {
    let clean = request_path
        .trim_start_matches('/')
        .split('?')
        .next()
        .unwrap_or_default();

    let relative = if clean.is_empty() {
        "index.html"
    } else {
        clean
    };
    Path::new(ROOT).join(relative)
}

fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "css" => "text/css; charset=utf-8",
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}
