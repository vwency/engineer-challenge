# k6_tests

### Available scripts

| Script             | Description                                                                                         |
| ------------------ | --------------------------------------------------------------------------------------------------- |
| `yarn build`       | Собирает проект (TypeScript → JavaScript) с помощью `build.js`                                      |
| `yarn test`        | Собирает проект и запускает нагрузочный тест через k6                                               |
| `yarn test:smoke`  | Быстрый smoke-тест (5 VUs, 30 секунд)                                                               |
| `yarn test:report` | Запускает тест и сохраняет результаты в `results.json`                                              |
| `yarn test:vv`     | Запускает тест, экспортирует summary в `summary.json` и выполняет кастомный анализ (`print_500.js`) |
