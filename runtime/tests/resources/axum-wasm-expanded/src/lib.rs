use futures::TryStreamExt;
use cyndra_next::{
    body::BoxBody,
    extract::BodyStream,
    response::{IntoResponse, Response},
};
use tracing::debug;

pub fn handle_request(req: cyndra_next::Request<BoxBody>) -> cyndra_next::response::Response {
    cyndra_next::block_on(app(req))
}

async fn app(request: cyndra_next::Request<BoxBody>) -> cyndra_next::response::Response {
    use cyndra_next::Service;

    let mut router = cyndra_next::Router::new()
        .route("/hello", cyndra_next::routing::get(hello))
        .route("/goodbye", cyndra_next::routing::get(goodbye))
        .route("/uppercase", cyndra_next::routing::post(uppercase));

    let response = router.call(request).await.unwrap();

    response
}

async fn hello() -> &'static str {
    debug!("in hello()");
    "Hello, World!"
}

async fn goodbye() -> &'static str {
    debug!("in goodbye()");
    "Goodbye, World!"
}

// Map the bytes of the body stream to uppercase and return the stream directly.
async fn uppercase(body: BodyStream) -> impl IntoResponse {
    debug!("in uppercase()");
    let chunk_stream = body.map_ok(|chunk| {
        chunk
            .iter()
            .map(|byte| byte.to_ascii_uppercase())
            .collect::<Vec<u8>>()
    });
    Response::new(cyndra_next::body::StreamBody::new(chunk_stream))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __cyndra_Axum_call(
    parts_fd: std::os::wasi::prelude::RawFd,
    body_fd: std::os::wasi::prelude::RawFd,
) {
    use cyndra_next::body::{Body, HttpBody};
    use std::io::{Read, Write};
    use std::os::wasi::io::FromRawFd;

    use cyndra_next::tracing_prelude::*;

    cyndra_next::tracing_registry()
        .with(cyndra_next::tracing_fmt::layer().without_time())
        .init();

    // file descriptor 3 for reading and writing http parts
    let mut parts_fd = unsafe { std::fs::File::from_raw_fd(parts_fd) };

    let reader = std::io::BufReader::new(&mut parts_fd);

    // deserialize request parts from rust messagepack
    let wrapper: cyndra_next::RequestWrapper = cyndra_next::from_read(reader).unwrap();

    // file descriptor 4 for reading and writing http body
    let mut body_stream = unsafe { std::fs::File::from_raw_fd(body_fd) };

    let mut reader = std::io::BufReader::new(&mut body_stream);
    let mut body_buf = Vec::new();
    reader.read_to_end(&mut body_buf).unwrap();

    let body = Body::from(body_buf);

    let request = wrapper
        .into_request_builder()
        .body(cyndra_next::body::boxed(body))
        .unwrap();

    let res = handle_request(request);

    let (parts, mut body) = res.into_parts();

    // wrap and serialize response parts as rmp
    let response_parts = cyndra_next::ResponseWrapper::from(parts)
        .into_rmp()
        .expect("failed to serialize response parts");

    // write response parts
    parts_fd.write_all(&response_parts).unwrap();

    // write body if there is one
    if let Some(body) = cyndra_next::block_on(body.data()) {
        body_stream.write_all(body.unwrap().as_ref()).unwrap();
    }
}
