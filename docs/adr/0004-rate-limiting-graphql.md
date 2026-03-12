# ADR-003: Rate Limiting для GraphQL операций

## Status
Accepted

## Date
2026-03-13

---

## Context

GraphQL gateway сервиса аутентификации подвержен риску злоупотреблений: брутфорс login,
массовая регистрация аккаунтов, спам запросов на восстановление пароля. Все операции
проходят через единый endpoint `POST /graphql` — необходима гранулярная защита
на уровне отдельных GraphQL операций с различными лимитами.

---

## Decision Drivers

- Разные операции требуют разных лимитов (login строже чем currentUser)
- Реальный IP пользователя через `X-Forwarded-For` от Istio с `xff_num_trusted_hops: 1`
- Переиспользование существующего Valkey инстанса без новых зависимостей
- Конфигурируемость лимитов через toml без перекомпиляции

---

## Considered Options

- Option 1: Istio EnvoyFilter + внешний rate limit сервис
- Option 2: Rate limiting на уровне GraphQL резолверов
- Option 3: Гибридный подход

---

## Decision

Выбран **Option 2: application-level rate limiting в GraphQL резолверах**.

---

## Pros and Cons of the Options

### Option 1: Istio EnvoyFilter + внешний rate limit сервис

**Pro:**
- Защита до попадания трафика в приложение

**Con:**
- Istio не умеет инспектировать тело GraphQL запроса из коробки
- Требует отдельного ext_authz сервиса парсящего GraphQL AST
- Высокая операционная сложность и latency overhead

---

### Option 2: Rate limiting в GraphQL резолверах

**Pro:**
- Полный доступ к контексту: операция, пользователь, IP
- Использует существующий Valkey — нет новых зависимостей
- Лимиты конфигурируются через toml без перекомпиляции

**Con:**
- Не защищает от DDoS на сетевом уровне
- Fixed Window Counter имеет edge case на границе окна

---

### Option 3: Гибридный подход

**Pro:**
- Istio ограничивает общий поток на `/graphql` endpoint, application-level даёт гранулярность per-operation

**Con:**
- Istio не видит GraphQL операции внутри тела запроса — защита per-mutation возможна только на application-level в любом случае

---

## Implementation

Алгоритм **Fixed Window Counter** в Valkey. Ключи `rate_limit:{operation}:{identifier}`,
TTL равен window_seconds, сбрасывается автоматически.

Реальный IP извлекается из `X-Forwarded-For` выставленного Istio Ingress Gateway.
На уровне Istio настроен `xff_num_trusted_hops: 1` — подделка заголовка клиентом невозможна.

---

## Consequences

### Positive
- Гранулярная защита per-operation без внешних сервисов
- Ключи изолированы префиксом `rate_limit:` от сессионных ключей в том же Valkey

### Negative
- При необходимости защиты от DDoS дополняется Istio глобальным лимитом (Option 3)

---

## References

- [ADR-001: Valkey](./0001-valkey-cache.md)
- [ADR-002: GraphQL Gateway](./0002-graphql-gateway.md)
- [Envoy xff_num_trusted_hops](https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/network/http_connection_manager/v3/http_connection_manager.proto#envoy-v3-api-field-extensions-filters-network-http-connection-manager-v3-httpconnectionmanager-xff-num-trusted-hops)
