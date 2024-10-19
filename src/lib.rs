pub type SimpleError = Box<dyn std::error::Error + Send + Sync>;
pub type SimpleResult<R> = Result<R, SimpleError>;

#[macro_export]
macro_rules! box_err {
    ($msg:expr) => {
        $crate::SimpleError::from($msg)
    };
}
