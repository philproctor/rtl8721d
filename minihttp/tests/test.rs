extern crate alloc;

use anyhow::Result;
use bare_io::Write;
use minihttp::*;

mod stdstack;
use stdstack::TestStack as stack;

#[test]
fn client_test() -> Result<()> {
    let http = Http::new(stack);
    let request = OutgoingRequest::new(
        "httpbin.org".into(),
        HttpMethod::GET,
        "/get?sometestparam=sometestvalue".into(),
        None,
    );
    let result = spin_on::spin_on(http.send_request(request));
    println!("Result: {}", result.unwrap().to_string());
    Ok(())
}

async fn do_one(req: HttpRequest) -> HttpResponse {
    let mut ms = MemoryStream::new(200);
    let path = req.path;
    write!(&mut ms, "First handler: {}", path).unwrap();
    let mut headers = HttpHeaders::new();
    headers.push(HttpHeader::new("", ""));
    HttpResponse {
        status: HttpStatus::OK,
        headers: HttpHeaders::new(),
        body: Some(ms.consume()),
    }
}

async fn do_two(req: HttpRequest) -> HttpResponse {
    let mut ms = MemoryStream::new(200);
    let path = req.path;
    write!(&mut ms, "Second handler: {}", path).unwrap();
    let mut headers = HttpHeaders::new();
    headers.push(HttpHeader::new("", ""));
    HttpResponse {
        status: HttpStatus::OK,
        headers: HttpHeaders::new(),
        body: Some(ms.consume()),
    }
}

minirouter! {
    <TestRouter>
    content {
        ("/" | "/index.html") => b"content".clone(),
    },
    api {
        ("/test") => do_one,
        ("/another") => do_two,
    }
}

#[test]
fn server_test() -> Result<()> {
    let http = Http::new(stack);
    spin_on::spin_on(http.server::<TestRouter>()).unwrap();
    Ok(())
}
