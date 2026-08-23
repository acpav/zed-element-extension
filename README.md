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
  - lambdas (`П -> Выражение`), null-forgiving `!`, optional chaining `?.`,
    `Тип<...>` type-of operator
  - annotations `@Имя`, comments `//` and `/* */`
- **LSP** via the [lsp-element-xbsl](https://github.com/acpav/lsp-element-xbsl)
  language server:
  - diagnostics: parse errors and missing tokens
  - document outline: methods (with parameters), types, constants, variables
  - completion: bilingual keywords (RU/EN) and identifiers declared in the file
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

### Language server

On the first `.xbsl` file the extension downloads `lsp-element-xbsl` from
[GitHub Releases](https://github.com/acpav/lsp-element-xbsl/releases) (see
`SERVER_VERSION` in `src/lib.rs` for the pinned tag) into the extension
working directory. Alternatively, put the server binary on `$PATH` — the
extension will use it as is.

To build the server from source: `cargo install --git
https://github.com/acpav/lsp-element-xbsl`.

## Structure

```
zed-element-extension-main/
├── extension.toml          # extension manifest (language, grammar, LSP)
├── Cargo.toml              # Rust extension code (zed_extension_api)
├── src/lib.rs              # downloads/launches the LSP server
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

- Semantic tokens, run/debug support.
- LSP: hover, go-to-definition.

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
  - лямбды (`П -> Выражение`), null-forgiving `!`, опциональный доступ `?.`,
    оператор `Тип<...>`
  - аннотации `@Имя`, комментарии `//` и `/* */`
- **LSP** через языковой сервер
  [lsp-element-xbsl](https://github.com/acpav/lsp-element-xbsl):
  - диагностика: ошибки парсинга и пропущенные токены
  - структура документа: методы (с параметрами), типы, константы, переменные
  - автодополнение: ключевые слова (RU/EN) и имена, объявленные в файле
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

### Языковой сервер

При первом открытии `.xbsl` расширение скачивает `lsp-element-xbsl` из
[GitHub Releases](https://github.com/acpav/lsp-element-xbsl/releases)
(пиннингованная версия — константа `SERVER_VERSION` в `src/lib.rs`) в
рабочий каталог расширения. Альтернативно можно положить бинарник сервера в
`$PATH` — расширение использует его как есть.

Сборка сервера из исходников: `cargo install --git
https://github.com/acpav/lsp-element-xbsl`.

## Структура

```
zed-element-extension-main/
├── extension.toml          # манифест расширения (язык, грамматика, LSP)
├── Cargo.toml              # Rust-код расширения (zed_extension_api)
├── src/lib.rs              # скачивание/запуск LSP-сервера
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

- Семантические токены, запуск/отладка.
- LSP: hover, переход к определению.

## Лицензия

MIT, см. [LICENSE](./LICENSE).
