use std::future::Future;
use std::pin::Pin;

use typed_path::Utf8TypedPathBuf;

use super::Fetch;

#[derive(Debug, Default)]
pub struct Tokio;

#[cfg(not(target_arch = "wasm32"))]
impl Fetch for Tokio {
    fn fetch(
        &self,
        path: Utf8TypedPathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        let fut = async { tokio::fs::read_to_string(path.into_string()).await };
        Box::pin(fut)
    }
}

#[cfg(target_arch = "wasm32")]
impl Fetch for Tokio {
    fn fetch(
        &self,
        _path: Utf8TypedPathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        todo!()
    }
}
