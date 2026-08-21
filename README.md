# zed-element-extension

A [Zed](https://zed.dev) editor extension with support for the **1C:Element** language (XBSL, `.xbsl` files).

Based on the Visual Studio Code extension [element-lang-vscode](https://github.com/avolkov-git/element-lang-vscode): syntax highlighting and language configuration.

## Features

- **Syntax highlighting** via the [tree-sitter-xbsl](https://github.com/acpav/tree-sitter-xbsl) grammar (Tree-sitter, no TextMate):
  - operators, declarations and modifiers: `если/иначе`, `пока`, `для`,
    `попытка/поймать`, `выбор/когда`, `метод`, `структура`,
    `перечисление`, `контракт`, `исключение`, `знч/пер/обз/исп`, `конст`,
    `импорт`, `возврат`, `выбросить`, `новый`, …
  - English keywords (`method`, `var`, `if`, `else`, …)
  - strings with escaping and interpolation (`"$Имя"`, `"%{Имя}"`),
    numbers, durations (`1д2ч3м4с5мс`), `Истина/Ложь/Неопределено`
  - typed literals (`Ресурс{...}`, `Время{00:00:00}`, …)
  - query blocks `Запрос{ ... }` with XBQL highlighting (keywords,
    aggregate functions, parameters `%Параметр`/`%{...}`)
  - annotations `@Имя`, comments `//` and `/* */`
- **Outline**: methods, types, constants, variables
- **Text objects**: `method` / `class` for Vim mode
- **Indentation**: auto-indent for compound statements, auto-dedent for
  `иначе`, `поймать`, `когда` and the terminating `;`
- **Brackets**: auto-close `{}`, `[]`, `()`, `""`, `''` (except inside strings
  and comments), bracket highlighting

## Installation

### Local development (dev extension)

1. Open the Extensions page in Zed, click `Install Dev Extension`, and select
   the `zed-element-extension` directory.
2. The grammar is fetched from the local `tree-sitter-xbsl` git repository
   (see `repository` in `extension.toml`). When you open a `.xbsl` file for the
   first time, Zed compiles the parser itself (requires `wasi-sdk`, which Zed
   downloads automatically).

## Structure

```
zed-element-extension-main/
├── extension.toml          # extension manifest
├── languages/xbsl/         # language configuration and Tree-sitter queries
│   ├── config.toml
│   ├── highlights.scm
│   ├── brackets.scm
│   ├── indents.scm
│   ├── outline.scm
│   ├── textobjects.scm
│   └── overrides.scm
├── examples/               # .xbsl examples
```

## Roadmap

- LSP server integration for XBSL (e.g.,
  [keyfire/xbsl](https://github.com/keyfire/xbsl)) via `language_servers` in
  the manifest and extension Rust code.
- Semantic tokens, run/debug support.

## License

MIT, see [LICENSE](./LICENSE).

---

# zed-element-extension (Русский)

Расширение для редактора [Zed](https://zed.dev) с поддержкой языка
**1С:Элемент** (XBSL, файлы `.xbsl`).

Основано на расширении Visual Studio Code
[element-lang-vscode](https://github.com/avolkov-git/element-lang-vscode):
подсветка синтаксиса, языковая конфигурация.

## Возможности

- **Подсветка синтаксиса** через грамматику
  [tree-sitter-xbsl](https://github.com/acpav/tree-sitter-xbsl) (Tree-sitter, без TextMate):
  - операторы, объявления и модификаторы: `если/иначе`, `пока`, `для`,
    `попытка/поймать`, `выбор/когда`, `метод`, `структура`,
    `перечисление`, `контракт`, `исключение`, `знч/пер/обз/исп`, `конст`,
    `импорт`, `возврат`, `выбросить`, `новый`, …
  - английские ключевые слова (`method`, `var`, `if`, `else`, …)
  - строки с экранированием и интерполяцией (`"$Имя"`, `"%{Имя}"`),
    числа, длительности (`1д2ч3м4с5мс`), `Истина/Ложь/Неопределено`
  - типизированные литералы (`Ресурс{...}`, `Время{00:00:00}`, …)
  - блоки запросов `Запрос{ ... }` с подсветкой XBQL (ключевые слова,
    агрегатные функции, параметры `%Параметр`/`%{...}`)
  - аннотации `@Имя`, комментарии `//` и `/* */`
- **Структура кода** (outline): методы, типы, константы, переменные
- **Навигация** (text objects): `method` / `class` для Vim-режима
- **Отступы**: авто-отступ для составных операторов, авто-выравнивание
  `иначе`, `поймать`, `когда` и завершающей `;`
- **Скобки**: авто-закрытие `{}`, `[]`, `()`, `""`, `''` (кроме строк и комментариев),
  подсветка парных скобок

## Установка

### Локальная разработка (dev extension)

1. Откройте страницу расширений (`Extensions`) в Zed и нажмите
   `Install Dev Extension`, выберите каталог `zed-element-extension`.
2. Грамматика загружается из локального git-репозитория `tree-sitter-xbsl`
   (см. `repository` в `extension.toml`). При первом открытии файла `.xbsl`
   Zed сам скомпилирует парсер (потребуется `wasi-sdk`, Zed скачает его
   автоматически).

## Структура

```
zed-element-extension-main/
├── extension.toml          # манифест расширения
├── languages/xbsl/         # конфигурация языка и Tree-sitter запросы
│   ├── config.toml
│   ├── highlights.scm
│   ├── brackets.scm
│   ├── indents.scm
│   ├── outline.scm
│   ├── textobjects.scm
│   └── overrides.scm
├── examples/               # примеры .xbsl
```

## Дальнейшее развитие

- Подключение LSP-сервера для XBSL (например,
  [keyfire/xbsl](https://github.com/keyfire/xbsl)) через `language_servers`
  в манифесте и Rust-код расширения.
- Семантические токены, запуск/отладка.

## Лицензия

MIT, см. [LICENSE](./LICENSE).
