; Comments
(comment) @comment

; Annotations
(annotation) @attribute

; Control-flow and expression keywords
[
  "если" "иначе" "пока" "для" "каждый" "попытка" "поймать"
  "возврат" "выбросить" "не" "и" "или" "как" "это" "этот"
  "выбор" "когда" "по" "из" "до" "новый"
  "if" "else" "while" "for" "each" "try" "catch"
  "return" "throw" "not" "and" "or" "as" "is" "this"
  "when" "select" "in" "to"
] @keyword

; Declaration keywords and storage modifiers
[
  "метод" "структура" "перечисление" "контракт" "исключение"
  "статический" "абстрактный" "знч" "пер" "обз" "исп" "конст"
  "method" "struct" "enum" "contract" "exception"
  "static" "abstract" "var" "const"
] @preproc

; Imports
(import_statement "импорт" @keyword)
(import_statement "import" @keyword)
(import_statement path: (qualified_identifier) @type)

; Declarations
(method_declaration name: (identifier) @function)
(type_declaration name: (identifier) @type)
(constant_declaration name: (identifier) @constant)
(variable_declaration name: (identifier) @variable)
(parameter name: (identifier) @variable.parameter)
(for_statement name: (identifier) @variable)
(catch_clause (identifier) @variable.parameter)

; Method calls
(call_expression
  callee: (postfix_expression (primary_expression (identifier) @function)))

(call_expression
  callee: (postfix_expression (member_expression property: (identifier) @function)))

; Types
(type_identifier) @type
(builtin_type) @type.builtin
(braced_literal name: (identifier) @type)

; Literals
(string_literal) @string
(escape_sequence) @string.escape
(interpolation) @string.special
(number_literal) @number
(duration_literal) @number
(boolean_literal) @constant
(undefined_literal) @constant

; XBQL query blocks
(query_keyword) @keyword
(query_aggregate) @function
(query_parameter) @variable.special
(query_identifier) @type

; Operators
[
  "=" "==" "!=" "<" ">" "<=" ">=" "+" "-" "*" "/" "%" "?" ":" "->" "|" "&" "<>"
] @operator

; Punctuation
[
  "(" ")" "[" "]" "{" "}"
] @punctuation.bracket

[
  ";" ","
] @punctuation.delimiter
