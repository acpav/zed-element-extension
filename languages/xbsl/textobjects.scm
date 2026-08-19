(method_declaration
  body: (declaration_body) @function.inside) @function.around

(type_declaration
  body: (declaration_body) @class.inside) @class.around
