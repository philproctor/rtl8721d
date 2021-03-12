pub use crate::prelude::*;
use minihttp::*;

pub async fn http_server() {
    let http = Http::new(LwipInterface);
    http.server::<RtlRouter>()
        .await
        .unwrap_or_else(|e| warn!("Failure! {:?}", e));
}

minirouter! {
    struct RtlRouter {
        content {
            ("/" | "/index.html") => b"content".clone(),
            // ("/cargo.toml") => *include_bytes!("../../Cargo.toml"),
        },
        api {}
    }
}
