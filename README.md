Реализовать на Rust простое REST API с одним единственным методом, который загружает изображения.

Требования:
- Возможность загружать несколько файлов.
- Возможность принимать multipart/form-data запросы.
- Возможность принимать JSON запросы с BASE64 закодированными изображениями.
- Возможность загружать изображения по заданному URL (изображение размещено где-то в интернете).
- Создание квадратного превью изображения размером 100px на 100px.

Временем и инструментом для выполнение тестового задания Вы не ограничены. Любые другие аспекты реализации, которые не указаны в требованиях, могут быть выполнены на Ваше усмотрение.

Следующее будет плюсом:
- Корректное завершение приложения при получении сигнала ОС (graceful shutdown).
- Dockerfile и docker-compose.yml, которые позволяют поднять приложение единой docker-compose up командой.
- Модульные тесты, функциональные тесты, CI интеграция (Travis CI, Circle CI, другие).
- Использование отдельной внешней библиотеки обработки изображений (к примеру, OpenCV) для демонстрации FFI.

Тестовое задание должно быть предоставлено в виде ссылки на публичный репозиторий (GitHub, BitBucket, GitLab) с исходным кодом. Приветствуется README.md файл с обзором реализации и/или шагами по запуску.


# Description

## Setting up

This program uses PostgreSQL database to store file locations and description. Pictures are stored in filesystem to avoid additional database overhead.

To run this program local PostgreSQL server has to be up. On systems that utilize systemd (most GNU/Linux distributions) this can be done by issuing command `sudo service postgresql start` if postgresql is installed


