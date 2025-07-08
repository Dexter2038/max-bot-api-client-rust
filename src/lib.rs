mod bot;
mod error;
pub mod responses;

pub use crate::bot::Bot;
pub use crate::error::ClientError;

#[cfg(test)]
mod tests {
    use crate::ClientError;
    use once_cell::sync::Lazy;

    static TOKEN: Lazy<String> =
        Lazy::new(|| std::env::var("MAX_BOT_TOKEN").expect("MAX_BOT_TOKEN not set"));

    #[tokio::test]
    async fn get_me() -> Result<(), ClientError> {
        use crate::Bot;

        let client = Bot::new(TOKEN.to_string());

        client.get_me().await?;
        Ok(())
    }
}
