use reqwest::Client;

use crate::{error::ClientError, responses::GetMeResponse};

/// Клиент для работы с Max Messenger Bot API.
/// Позволяет выполнять синхронные запросы к API с использованием HTTP агента `ureq`.
pub struct Bot {
    client: Client,
    base_url: String,
    access_token: String,
}

impl Bot {
    /// Создаёт новую сессию бота с указанным токеном доступа и базовым URL по умолчанию.
    ///
    /// # Аргументы
    ///
    /// * `access_token` — токен доступа бота, используемый для авторизации в API.
    ///
    /// # Пример
    ///
    /// ```
    /// let bot = Bot::new("your_access_token");
    /// ```
    pub fn new(access_token: impl Into<String>) -> Self {
        let client = match Client::builder().https_only(true).build() {
            Ok(client) => client,
            Err(_) => Client::new(),
        };
        Self {
            client,
            base_url: "https://botapi.max.ru".into(),
            access_token: access_token.into(),
        }
    }

    /// Создаёт новую сессию бота с указанным токеном доступа и произвольным базовым URL.
    ///
    /// Используется для тестирования или работы с альтернативными серверами API.
    ///
    /// # Аргументы
    ///
    /// * `access_token` — токен доступа бота.
    /// * `base_url` — базовый URL API.
    ///
    /// # Пример
    ///
    /// ```
    /// let bot = Bot::with_base_url("your_access_token", "https://custom.api.url");
    /// ```
    pub fn with_base_url(access_token: impl Into<String>, base_url: impl Into<String>) -> Self {
        let client = match Client::builder().https_only(true).build() {
            Ok(client) => client,
            Err(_) => Client::new(),
        };
        Self {
            client,
            base_url: base_url.into(),
            access_token: access_token.into(),
        }
    }

    /// Получает информацию о текущем боте.
    ///
    /// Выполняет запрос к методу `/me` API и возвращает данные о боте.
    ///
    /// # Ошибки
    ///
    /// Возвращает `ClientError` при сетевых ошибках, ошибочном HTTP статусе или ошибках парсинга.
    ///
    /// # Пример
    ///
    /// ```
    /// let bot_info = bot.get_me()?;
    /// println!("Bot ID: {}", bot_info.user_id);
    /// ```
    pub async fn get_me(&self) -> Result<GetMeResponse, ClientError> {
        let url = format!("{}/me?access_token={}", self.base_url, self.access_token);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(ClientError::StatusError(response.status().as_u16()));
        }

        let body = response.text().await?;

        let result = serde_json::from_str::<GetMeResponse>(&body)?;

        Ok(result)
    }
}
