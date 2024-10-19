pub type SimpleResult<R> = Result<R, Box<dyn std::error::Error + Send + Sync>>;

#[macro_export]
macro_rules! box_err {
    ($msg:expr) => {
        Box::<dyn std::error::Error + Send + Sync>::from($msg)
    };
}
