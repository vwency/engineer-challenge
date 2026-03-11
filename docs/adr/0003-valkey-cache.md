# ADR-001: Выбор Valkey как решения для кэширования сессионных данных

## Status
Accepted

## Date
2026-03-11

---

## Context

В рамках сервиса аутентификации на базе Ory Kratos возникла необходимость кэширования профилей пользователей (`UserProfile`) для снижения нагрузки на Kratos при каждом запросе `GET /sessions/whoami`. Запрос `get_current_user` выполняется при каждом GraphQL-запросе требующем аутентификации, что создаёт избыточную нагрузку на Kratos и увеличивает latency.

Требования к решению:

- TTL-based инвалидация кэша
- Инвалидация при logout
- Совместимость с существующим Rust-стеком
- Приемлемая лицензия для коммерческого использования
- Простота операционного обслуживания

Рассматривались три кандидата: **Redis**, **Valkey**, **Aerospike**.

---

## Decision Drivers

- Лицензионная чистота для коммерческого продукта
- Совместимость с крейтом `redis` в Rust без изменения кода
- Зрелость и поддержка сообщества
- Операционная простота (Docker, минимальная конфигурация)
- Производительность для сценария key-value кэширования строк

---

## Considered Options

- Option 1: Redis
- Option 2: Valkey
- Option 3: Aerospike

---

## Decision

Выбран **Valkey**.

---

## Pros and Cons of the Options

### Option 1: Redis

**Pro:**
- Зрелое решение с многолетней историей
- Огромная экосистема и документация
- Широкая поддержка в крейтах Rust

**Con:**
- Начиная с версии 7.4 Redis перешёл на двойную лицензию SSPL / RSALv2
- SSPL несовместима с коммерческим использованием без покупки коммерческой лицензии у Redis Ltd.
- Vendor lock-in: единственный вендор Redis OSS — Redis Ltd.
- OSI не признаёт SSPL как open source лицензию
- Риск дальнейшего ужесточения условий лицензирования

---

### Option 2: Valkey

**Pro:**
- Форк Redis 7.2 под лицензией BSD-3-Clause — полностью открытой и коммерчески приемлемой
- Поддерживается Linux Foundation
- Мейнтейнеры: AWS, Google Cloud, Oracle, Ericsson, Snap
- Полная совместимость с Redis-протоколом — крейт `redis` в Rust работает без изменений
- Valkey 8.x добавил многопоточную обработку I/O, превосходя Redis 7.x по throughput
- Официальный образ `valkey/valkey` на Docker Hub
- Активное развитие: релизы выходят чаще чем у Redis OSS

**Con:**
- Меньший исторический track record по сравнению с Redis
- Меньше готовых managed-решений у облачных провайдеров (хотя AWS уже предоставляет Valkey в ElastiCache)

---

### Option 3: Aerospike

**Pro:**
- Высокая производительность при работе с большими объёмами данных
- Поддержка SSD-хранилища из коробки
- Подходит для mission-critical сценариев с терабайтами данных

**Con:**
- Избыточен для сценария кэширования сессионных строк
- Отсутствует нативная поддержка в популярных Rust-крейтах, требуется кастомный клиент
- Значительно более сложная операционная модель
- Community edition имеет ограничения; enterprise edition — платный
- Нет встроенной TTL-семантики уровня Redis/Valkey для простых key-value операций
- Высокий порог входа для DevOps

---

## Consequences

### Positive
- Кодовая база не требует изменений — крейт `redis` с `tokio-comp` и `connection-manager` работает с Valkey без модификаций
- Лицензионная чистота проекта обеспечена
- Инвалидация кэша при logout реализована через `DEL user_profile:{token}`
- TTL управляется через конфиг (`cache_ttl_secs`, default 300s)
- Горизонтальное масштабирование возможно через Valkey Cluster (совместим с Redis Cluster протоколом)

### Negative
- При миграции на managed-cloud потребуется проверить поддержку Valkey у провайдера
- Необходимо отслеживать совместимость новых версий Valkey с крейтом `redis`

### Neutral
- Мониторинг через стандартные Redis-совместимые метрики (`INFO`, `MONITOR`)
- Существующие инструменты (`redis-cli`, `RedisInsight`) работают с Valkey без изменений

---

## References

- [Valkey GitHub](https://github.com/valkey-io/valkey)
- [Linux Foundation Valkey](https://www.linuxfoundation.org/press/linux-foundation-launches-open-source-valkey-community)
- [Redis License Change Announcement](https://redis.com/blog/redis-adopts-dual-source-available-licensing/)
- [AWS ElastiCache Valkey Support](https://aws.amazon.com/elasticache/valkey/)
- [Valkey 8.0 Performance Benchmarks](https://valkey.io/blog/valkey-8-0-0-ga/)
