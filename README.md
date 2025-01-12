# 📺 EPGenius - Интеллектуальный Гид по IPTV 🤖 (Автор: Редаль Габдулхаков)

[![Rust](https://img.shields.io/badge/rust-v1.70+-orange.svg)](https://www.rust-lang.org/)
[![Warp](https://img.shields.io/badge/warp-v0.3-blue.svg)](https://crates.io/crates/warp)
[![Dioxus](https://img.shields.io/badge/dioxus-v0.5-brightgreen.svg)](https://dioxus.rs/)
[![SQLite](https://img.shields.io/badge/sqlite-3-yellowgreen.svg)](https://www.sqlite.org/index.html)


## 🚧 Внимание: Проект в Разработке! 🚧

Этот проект находится в активной разработке и является прототипом интеллектуальной системы для анализа и рекомендаций контента IPTV.  Функциональность может меняться, а API не является стабильным. **Не рекомендуется использовать в production.**

## 🤔 О проекте

**EPGenius** - это ваш персональный телегид, разработанный для того, чтобы помочь вам сориентироваться в огромном мире IPTV. Он использует возможности машинного обучения, чтобы анализировать EPG (Electronic Program Guide) и предоставлять персонализированные рекомендации, основанные на ваших интересах и предпочтениях.

### Основные возможности:
*   **💾 Сохранение и управление пользовательскими интересами:** Вы можете добавить интересующие вас жанры, ключевые слова и временные рамки.
*   **🤖 Рекомендации на основе нейросети:** Сервис использует нейросеть для анализа телепередач и предоставления рекомендаций. 
*   **📅 Ежедневное обновление EPG:**  Система автоматически обновляет данные телепрограммы.
*   **🗄️ Хранение данных:** Данные о пользователях, интересах и EPG хранятся в локальной базе данных SQLite.
*   **✨ Современный веб-интерфейс:** Удобный и отзывчивый UI, разработанный на Dioxus.

## ⚙️ Технологии

### Серверная часть:
*   **Rust:** Язык программирования для высокой производительности и безопасности.
*   **Warp:** Фреймворк для создания веб-сервисов.
*   **Rusqlite:** Библиотека для работы с SQLite.
*   **Tokio:** Асинхронный рантайм для Rust.

### Клиентская часть:
*   **Dioxus:** Библиотека для создания интерактивных веб-интерфейсов с использованием Rust.
*   **HTML/CSS:** Для разметки и стилизации интерфейса.

## 🚀 Запуск проекта

### Серверная часть

1.  Убедитесь, что у вас установлен Rust.
2.  Клонируйте репозиторий:
    ```bash
    git clone https://github.com/redal499/EPGenius.git
    cd EPGenius.git
    ```
3.  Запустите сервер:
    ```bash
    cargo run
    ```
    Сервер запустится по адресу `http://127.0.0.1:3030`.

### Клиентская часть
1.  Убедитесь, что у вас установлен Rust и Dioxus-cli v0.6.0
2.  Перейдите в папку с клиентской частью
3.  Запустите проект:
    ```bash
    dx serve
    ```
    Сервер запустится по адресу `http://127.0.0.1:8080`.

## 📋 ТЗ (Кратко)

### Цель проекта
Создание прототипа интеллектуальной системы для анализа телепрограмм (EPG) и предоставления персонализированных рекомендаций пользователям IPTV на основе их предпочтений.

### Основные требования
*   Автоматическая загрузка и анализ EPG.
*   Интеграция с БД для хранения информации о пользователях, их интересах и телепрограммах.
*   Рекомендации на основе нейросети.
*   Управление личными кабинетами пользователей.
*   Простой и удобный интерфейс.

### Дополнительные возможности
*  Уведомления о предстоящих передачах.
*   Добавление программ в избранное.

## 🛠️ Структура проекта
```
epgenius/
├── src/
│   ├── db.rs       # Модуль работы с БД
│   ├── handlers.rs # Обработчики запросов
│   ├── models.rs   # Модели данных
│   ├── routes.rs   # Определение маршрутов API
│   ├── main.rs     # Основной файл приложения
|   └── mod.rs      # Главный модуль клиентской части
│   
├── assets/
    └── styling/    # Стили css
    └── images/      # Иконки
    
├── components/     # Компоненты клиентской части
    ├── account.rs  # Компонент аккаунта
    ├── generation.rs # Компонент генерации
    ├── hero.rs     # Главный экран
    ├── interests.rs # Экран интересов
    ├── navbar.rs   # Компонент навигации
    └── tvgid.rs    # Компонент ТВ-гида
    
├── Cargo.toml     # Файл конфигурации Rust
└── README.md       # Этот файл
```

## 🛣️ Планы на будущее

*   ✅ Базовая авторизация и регистрация пользователей.
*   ✅ Сохранение и управление интересами пользователя.
*   ✅ Отображение EPG.
*   ✅ Генерация рекомендаций.
*   ⬜  Интеграция с внешними API для получения дополнительной информации о фильмах и передачах.
*   ⬜  Улучшенная логика рекомендаций на основе нейросети.
*   ⬜  Реализация системы уведомлений.
*   ⬜  Добавление избранного.
*   ⬜  Более проработанный и адаптивный дизайн.

## 🤝 Contributing

Если у вас есть идеи, как улучшить этот проект, или вы хотите принять участие в разработке, пожалуйста, не стесняйтесь создавать issue или pull request!

## 📜 Лицензия

Этот проект распространяется под лицензией MIT.

---
Если у вас есть какие-либо вопросы или предложения, дайте мне знать!
