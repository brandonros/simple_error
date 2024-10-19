use std::error::Error;

pub type SimpleResult<R> = Result<R, Box<dyn Error + Send + Sync>>;
