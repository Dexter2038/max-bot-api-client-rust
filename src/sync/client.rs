use ureq::Agent;

use crate::sync::{error::ClientError, responses::GetMeResponse};

/// Клиент для работы с Max Messenger Bot API.
/// Позволяет выполнять синхронные запросы к API с использованием HTTP агента `ureq`.
pub struct Client {
    agent: Agent,
    base_url: String,
    access_token: String,
}

impl Client {
    /// Создаёт новый клиент с указанным токеном доступа и базовым URL по умолчанию.
    ///
    /// # Аргументы
    ///
    /// * `access_token` — токен доступа бота, используемый для авторизации в API.
    ///
    /// # Пример
    ///
    /// ```
    /// let client = Client::new("your_access_token");
    /// ```
    pub fn new(access_token: impl Into<String>) -> Self {
        let cfg = Agent::config_builder()
            .http_status_as_error(false)
            .https_only(true)
            .build();
        Self {
            agent: Agent::new_with_config(cfg),
            base_url: "https://botapi.max.ru".into(),
            access_token: access_token.into(),
        }
    }

    /// Создаёт новый клиент с указанным токеном доступа и произвольным базовым URL.
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
    /// let client = Client::with_base_url("your_access_token", "https://custom.api.url");
    /// ```
    pub fn with_base_url(access_token: impl Into<String>, base_url: impl Into<String>) -> Self {
        let cfg = Agent::config_builder().https_only(true).build();
        Self {
            agent: Agent::new_with_config(cfg),
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
    /// let bot_info = client.get_me()?;
    /// println!("Bot ID: {}", bot_info.user_id);
    /// ```
    pub fn get_me(&self) -> Result<GetMeResponse, ClientError> {
        let url = format!("{}/me?access_token={}", self.base_url, self.access_token);
        let response = self.agent.get(&url).call()?;

        if !response.status().is_success() {
            return Err(ClientError::StatusError(response.status().as_u16()));
        }

        let body = response.into_body().read_to_string()?;

        let result = serde_json::from_str::<GetMeResponse>(&body)?;

        Ok(result)
    }
}
