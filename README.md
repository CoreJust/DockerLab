# Docker: Докеризация приложения
## Лабораторная №4 по предмету Сети и Телекоммуникации

## Цель лабораторной

Цель лабораторной: собрать из исходного кода и запустить в докере рабочее приложение с базой данных (любое опенсорс - Java, python/django/flask, golang).

1. Образ должен быть легковесным.
2. Использовать базовые легковесные образы - alpine.
3. Вся конфигурация приложения должна быть через переменные окружения.
4. Статика (зависимости) должна быть внешним томом `volume`.
5. Создать файл `docker-compose` для старта и сборки.
6. В `docker-compose` нужно использовать базу данных (postgresql, mysql, mongodb etc.).
7. При старте приложения должно быть учтено выполнение автоматических миграций.
8. Контейнер должен запускаться от непривилегированного пользователя.
9. После установки всех нужных утилит, должен очищаться кеш.

## Описание работы

Разработано простое приложение на языке Rust. В качестве ORM используется SeaORM на основе rustls и async-std, для миграций используется CLI для лежащего в основе SeaORM SQLX. В качестве серверного web-фреймворка используется легковесная библиотека Tide на основе async-std. Для контейнеризации был использован Docker и для управления мультиконтейнерным приложением - Docker Compose.

## Структура проекта

- **Dockerfile**: Инструкции для Docker-образа.
- **docker-compose.yml**: Конфигурационный файл Docker Compose, управляющий многоконтейнерным приложением. В частности, содержит конфигурацию PostgresQL.
- **src/**: Исходные коды приложения.
- **Cargo.lock and Cargo.toml**: Файлы проекта.
- **migrations/**: Директория с файлами миграций.

## Установка и запуск
- Для сборки контейнеров и их запуска:
```bash
docker-compose up --build
```
(Можно использовать флаг -d для запуска в фоновом режиме)
- Проверка состояния:
```bash
docker-compose ps
```
- Остановка:
```bash
docker-compose stop
```
- Запуск:
```bash
docker-compose start
```
- Остановка и очистка всех созданных томов и контейнеров:
```bash
docker-compose down
```
