#[macro_export]
macro_rules! yield_nb {
    ( $( $nb:tt )* ) => {
        loop {
            match ( $( $nb )* ) {
                Err(embedded_nal::nb::Error::WouldBlock) => {
                    futures_lite::future::yield_now().await
                }
                Err(embedded_nal::nb::Error::Other(e)) =>
                {
                    #[allow(unreachable_code)]
                    break Err(e)
                }
                Ok(x) => break Ok(x),
            }
        }
    };
}
