use crate::error::ReviseResult;

pub mod gemini;

pub(crate) trait AI<T> {
    async fn generate_response(&self, input: &str) -> ReviseResult<T>;
}
