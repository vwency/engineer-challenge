# Auth Service

## Запуск
```bash
make up
```

## Функционал

1. Регистрация
2. Авторизация
3. Смена пароля
4. Восстановление по почте

### Trade-offs

1. Есть дублирование стилей/tsx. (скорость прототипирования)
2. Использование redux. (скорость прототипирования + архитектура)
3. Webpack (HMR, hot-reload)
4. Нет подтверждения пароля по почте при регистрация.(время отладки)
5. Ratelimitingless (Поскольку frontend имеет 1 ip нужно делать каждый раз проброс ip-пользователя через graphql к backend ИЛИ делать на proxy/ingress/loadbalancer лимит для маршрутов/запросов, если запросы к frontend будут идти через proxy, такое сделать не получиться)

## ADR

### [backend](./backend)

GraphQL поддерживает `Set-Cookies`.
Паттерны **DDD** и **DI**.
Ory экосистема

### [frontend](./frontend)

Монорепозиторий на **webpack** (поддержка HMR), **Nx**, **Next.js**.
**Redux**

### Проблемные места

1. После login, registeration нет редиректов на homepage.
2. Нету rate-limiting.
3. Hardcode

### Continue

1. GitOps чтение новых helm релизов, из применение.
2. Локальный раннер github actions.
3. Coverage тесты в ci, codecov, SonarQube
