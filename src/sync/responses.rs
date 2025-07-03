use serde::Deserialize;

/// Ответ на метод `/me` — информация о текущем боте.
#[derive(Deserialize)]
pub struct GetMeResponse {
    /// ID пользователя.
    pub user_id: i64,

    /// Отображаемое имя пользователя.
    pub first_name: String,

    /// Отображаемая фамилия пользователя.
    pub last_name: Option<String>,

    /// Отображаемое имя пользователя.
    /// Устаревшее поле, рекомендуется использовать `first_name`.
    #[deprecated = "Используйте `first_name` вместо этого поля"]
    pub name: Option<String>,

    /// Уникальное публичное имя пользователя (может быть null).
    pub username: Option<String>,

    /// Флаг, указывающий, что пользователь — бот.
    pub is_bot: bool,

    /// Время последней активности (Unix-время в миллисекундах).
    pub last_activity_time: i64,

    /// Описание пользователя (может быть null).
    pub description: Option<String>,

    /// URL аватара.
    pub avatar_url: Option<String>,

    /// URL аватара большего размера.
    pub full_avatar_url: Option<String>,

    /// Список поддерживаемых команд (до 32).
    pub commands: Vec<BotCommand>,
}

/// Команда бота.
#[derive(Deserialize)]
pub struct BotCommand {
    /// Название команды (1–64 символа).
    pub name: String,

    /// Описание команды (опционально).
    pub description: Option<String>,
}
