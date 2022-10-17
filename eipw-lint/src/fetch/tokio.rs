use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use super::Fetch;

#[derive(Debug, Default)]
pub struct Tokio;

#[cfg(not(target_arch = "wasm32"))]
impl Fetch for Tokio {
    fn fetch(
        &self,
        path: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        let fut = async { tokio::fs::read_to_string(path).await };
        Box::pin(fut)
    }
}

#[cfg(target_arch = "wasm32")]
impl Fetch for Tokio {
    fn fetch(
        &self,
        _path: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>> {
        todo!()
    }
}
