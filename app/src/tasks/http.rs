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
            // ("/" | "/index.html") => *include_bytes!("../../content/index.html"),
            // ("/js/app.js") => *include_bytes!("../../content/js/app.js"),
            // ("/js/vendors.js") => *include_bytes!("../../content/js/vendors.js"),
            // ("/css/app.css") => *include_bytes!("../../content/css/app.css"),
            ("/css/vendors.css") => *include_bytes!("../../content/css/vendors.css"),
        },
        api {}
    }
}

