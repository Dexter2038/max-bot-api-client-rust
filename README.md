# `max-bot-api-client-rust`

**Клиентская библиотека для Max Messenger Bot API на Rust с поддержкой синхронного и асинхронного режимов.**

Этот crate предоставляет типобезопасный и удобный интерфейс для работы с Max Bot API, позволяя разрабатывать ботов на Rust с использованием как async, так и sync подходов.

---

### ✨ Возможности

- Обёртка над всеми основными эндпоинтами Max Bot API.
- Асинхронный интерфейс на базе \[`reqwest`] и \[`tokio`].
- Синхронный интерфейс с использованием \[`ureq`] (или другой sync-библиотеки).
- Полная сериализация/десериализация через \[`serde`] и строгие структуры.
- Поддержка long polling и webhooks (в планах).
- Удобные модели ошибок и типизация.

---

### 📦 Установка

```toml
[dependencies]
max-bot-api-client = "0.1"
```

---

### 🚀 Пример использования (async)

```rust
use max_bot_api_client::async_api::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("YOUR_BOT_TOKEN");
    let me = client.get_me().await?;
    println!("Бот: {} ({})", me.name, me.username);
    Ok(())
}
```

---

### 🚀 Пример использования (sync)

```rust
use max_bot_api_client::sync_api::Client;

fn main() -> anyhow::Result<()> {
    let client = Client::new("YOUR_BOT_TOKEN");
    let me = client.get_me()?;
    println!("Бот: {} ({})", me.name, me.username);
    Ok(())
}
```

---

### 📄 Ссылки

- [Официальная документация Max Bot API (RU)](https://dev.max.ru/docs-api/)

### ©️ Лицензия

Этот проект распространяется под лицензией Mozilla Public License 2.0 (MPL-2.0).
Вы можете использовать crate в закрытых (коммерческих) проектах, но любые изменения кода библиотеки должны быть открыты в соответствии с условиями MPL.
