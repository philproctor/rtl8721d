use crate::common::*;
use alloc::boxed::Box;
use core::{future, pin::Pin};

pub trait HttpHandler {
    fn handle(request: HttpRequest)
        -> Option<Pin<Box<dyn future::Future<Output = HttpResponse>>>>;
}

#[macro_export]
macro_rules! minirouter {
    (struct $name:ident {
        content { $( ( $( $content_path:tt )* ) => $content:expr ,)* } ,
        api { $( ( $( $path:tt )* ) => $handler:expr ,)* }
    }) => {
        pub struct $name {}

        impl HttpHandler for $name {
            fn handle(
                req: HttpRequest,
            ) -> Option<core::pin::Pin<alloc::boxed::Box<dyn futures_lite::future::Future<Output = HttpResponse>>>> {
                let topin = Box::new(async move {
                    let resp: HttpResponse = match req.path.as_str() {
                        $( $( $content_path )* => {
                            HttpResponse {
                                status: HttpStatus::OK,
                                headers: HttpHeaders::new(),
                                body: Some( $content .into() ),
                            }
                        }, )*
                        $( $( $path )* => {
                            $handler (req).await
                        }, )*
                        _ => {
                            HttpResponse {
                                status: HttpStatus::NotFound,
                                headers: HttpHeaders::new(),
                                body: Some(alloc::vec![]),
                            }
                        },
                    };
                    resp
                });
                return Some(unsafe { core::pin::Pin::new_unchecked(topin) });
            }
        }
    };
}
