mod r#async;
mod sync;

/// Модуль с синхронным API клиента Max Messenger Bot.
/// Доступен при включённой фиче `"sync"`.
#[cfg(feature = "sync")]
pub mod sync_api {
    pub use crate::sync::client::Client;
    pub use crate::sync::error::ClientError;
    pub mod responses {
        pub use crate::sync::responses::*;
    }
}

/// Модуль с асинхронным API клиента Max Messenger Bot.
/// Доступен при включённой фиче `"async"`.
#[cfg(feature = "async")]
pub mod async_api {}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    static TOKEN: Lazy<String> = Lazy::new(|| {
        std::env::var("MAX_BOT_API_ACCESS_TOKEN").expect("MAX_BOT_API_ACCESS_TOKEN not set")
    });

    #[test]
    fn get_me() {
        use crate::sync_api::Client;

        let client = Client::new(TOKEN.to_string());

        client.get_me().unwrap();
    }
}
