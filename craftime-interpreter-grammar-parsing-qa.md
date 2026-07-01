# Crafting Interpreters Grammar Parsing Q&A

Source discussed: <https://craftinginterpreters.com/parsing-expressions.html>

## 1. What goes wrong if the grammar does not define separate rules for each precedence level?

If all expression forms are stuffed into one generic `expression` rule, the grammar does not encode precedence. That means an operand can be any expression, even one with a lower-precedence operator.

For example:

```lox
6 / 3 - 1
```

Without precedence encoded in the grammar, this could parse as either:

```lox
(6 / 3) - 1
```

or:

```lox
6 / (3 - 1)
```

Those are different syntax trees and can produce different results.

The same problem appears with unary operators:

```lox
-a * b
```

If unary is defined like:

```bnf
unary -> "-" expression
```

then `-a * b` could parse as:

```lox
-(a * b)
```

But normal precedence says unary binds tighter, so it should parse as:

```lox
(-a) * b
```

Crafting Interpreters fixes this by stratifying the grammar:

```bnf
expression -> equality
equality   -> comparison
comparison -> term
term       -> factor
factor     -> unary
unary      -> primary
primary    -> ...
```

Each rule only consumes operands from the next tighter level, so lower-precedence expressions cannot accidentally become operands of higher-precedence operators.

## 2. What would parser code likely look like if precedence levels were not separated?

A naive flat parser might look like:

```java
private Expr expression() {
  Expr expr = unary();

  while (match(BANG_EQUAL, EQUAL_EQUAL,
               GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,
               PLUS, MINUS,
               SLASH, STAR)) {
    Token operator = previous();
    Expr right = unary();
    expr = new Expr.Binary(expr, operator, right);
  }

  return expr;
}
```

This parses all operators left-to-right at the same precedence. So:

```lox
1 + 2 * 3
```

would incorrectly parse as:

```lox
(1 + 2) * 3
```

instead of:

```lox
1 + (2 * 3)
```

Another naive parser might try:

```java
private Expr expression() {
  Expr left = expression();

  if (match(PLUS, MINUS, STAR, SLASH)) {
    Token operator = previous();
    Expr right = expression();
    return new Expr.Binary(left, operator, right);
  }

  return primary();
}
```

But that is left-recursive in a recursive-descent parser. `expression()` calls itself before consuming any token, causing infinite recursion / stack overflow.

## 3. If not using layered recursive-descent rules, which precedence mechanism is easiest?

Practical ranking:

1. Layered recursive descent: easiest when following Crafting Interpreters.
2. Precedence climbing: easiest compact alternative for hand-written parsers.
3. Pratt parsing: more flexible and elegant, but harder to learn initially.
4. Shunting-yard: good for calculator-style parsers, less natural for rich AST parsers.

Short version:

```text
Learning recursive descent: stratified grammar
Alternative with least code: precedence climbing
Best long-term expression parser: Pratt
Calculator/parser exercise: shunting-yard
```

## 4. What does Nushell use?

Repository inspected:

```text
/home/chz/projects/rust_online_code/nushell
```

Nushell does not use Crafting Interpreters-style functions like:

```text
equality() -> comparison() -> term() -> factor()
```

Instead, it uses a custom stack-based operator-precedence parser, closest to a simplified shunting-yard / shift-reduce approach.

Key evidence:

- `/home/chz/projects/rust_online_code/nushell/crates/nu-parser/src/parse_expressions.rs:1208`
  defines `parse_math_expression`.
- `/home/chz/projects/rust_online_code/nushell/crates/nu-parser/src/parse_expressions.rs:1215`
  comments describe an expression stack that grows under increasing precedence and collapses under decreasing or sustained precedence.
- `/home/chz/projects/rust_online_code/nushell/crates/nu-parser/src/parse_expressions.rs:1226`
  uses `expr_stack`.
- `/home/chz/projects/rust_online_code/nushell/crates/nu-parser/src/parse_expressions.rs:1295`
  reads `op.precedence()`.
- `/home/chz/projects/rust_online_code/nushell/crates/nu-parser/src/parse_expressions.rs:1352`
  special-cases power `**` as right-associative.
- `/home/chz/projects/rust_online_code/nushell/crates/nu-protocol/src/ast/operator.rs:256`
  defines numeric operator precedence.

Conclusion:

```text
Nushell = custom stack-reduction precedence parser
Closest family = shunting-yard-like / operator-precedence shift-reduce
Not Pratt
Not canonical precedence climbing
Not layered recursive descent
```

## 5. What does `new-nu-parser` use?

Repository inspected:

```text
/home/chz/projects/rust_online_code/new-nu-parser
```

`new-nu-parser` uses almost the same conceptual algorithm as Nushell, but simplified.

Key evidence:

- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:379`
  defines `ASSIGNMENT_PRECEDENCE`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:381`
  defines `AstNode::precedence()`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:431`
  has `Parser::expression()` delegate to `math_expression(false)`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:481`
  defines `Parser::math_expression`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:483`
  creates `expr_stack`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:550`
  reduces while `op_prec <= last_prec`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:580`
  drains the stack into `AstNode::BinaryOp`.
- `/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:898`
  maps tokens to operator AST nodes.

Important difference from Nushell:

Nushell special-cases `**` as right-associative. `new-nu-parser` appears to reduce on `op_prec <= last_prec` for all operators, so `Pow` currently looks left-associative unless handled elsewhere.

## 6. Is it easy to change `new-nu-parser` to use recursive-descent precedence levels?

It is moderate, not trivial.

The parser is already recursive descent for most syntax. The part to replace is the expression precedence routine:

```text
/home/chz/projects/rust_online_code/new-nu-parser/src/parser.rs:481
```

Current shape:

```rust
fn math_expression(&mut self, allow_assignment: bool) -> AssignmentOrExpression {
    let mut expr_stack = Vec::<(NodeId, NodeId)>::new();
    let mut last_prec = 1000000;
    ...
}
```

A Crafting Interpreters-style rewrite would likely introduce functions like:

```text
expression     -> assignment
assignment     -> or ("=" assignment_or_pipeline)?
or             -> xor ("or" xor)*
xor            -> and ("xor" and)*
and            -> comparison ("and" comparison)*
comparison     -> term (("==" | "!=" | "<" | "in" | "++") term)*
term           -> factor (("+" | "-") factor)*
factor         -> power (("*" | "/" | "//" | "mod") power)*
power          -> unary ("**" power)?
unary          -> "not" unary | primary
primary        -> simple_expression(...)
```

Why it is feasible:

- Operator precedence is already explicit in `AstNode::precedence()`.
- Operator parsing is centralized in `operator()`.
- `simple_expression()` already handles primary/atom-like expressions.
- `expression()` already funnels into `math_expression(false)`.

Why it is not trivial:

- Assignment has special behavior: the RHS can be a pipeline.
- Pipeline parsing depends on `math_expression(true)`.
- Operator spacing diagnostics are centralized in the current loop.
- Snapshot tests may change, especially around `**` associativity.

Recommendation:

If the goal is educational clarity and Crafting Interpreters alignment, layered recursive descent is reasonable.

If the goal is smallest correct refactor, precedence climbing is probably easier because it can reuse the existing numeric precedence table.
