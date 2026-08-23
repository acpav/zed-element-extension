; Bodies of declarations and compound statements are indented by one level
; relative to their start line. The terminating `;` is captured with @end so
; its line stays at the parent level; branch keywords (иначе/поймать/когда)
; are dedented via the @start.* captures below together with
; `decrease_indent_patterns` in config.toml.

(declaration_body ";" @end) @indent

(if_statement ";" @end) @indent
(while_statement ";" @end) @indent
(for_statement ";" @end) @indent
(try_statement ";" @end) @indent
(select_statement ";" @end) @indent

; Labeled start positions used together with `decrease_indent_patterns` in
; config.toml to dedent branch keywords (иначе/поймать/когда) and the
; statement terminator `;` to the level of the statement they belong to.
(if_statement) @start.if
(else_clause) @start.else
(while_statement) @start.while
(for_statement) @start.for
(try_statement) @start.try
(catch_clause) @start.catch
(select_statement) @start.select
(when_clause) @start.case
(method_declaration) @start.method
(type_declaration) @start.type
