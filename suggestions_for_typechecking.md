# Codex

## Verdict

Crafting Interpreters got you to the right point: after chapter 11, your resolver is already a semantic-analysis pass, which is exactly where a type checker belongs. Your current runtime type checks live in `src/eval.rs`, while the resolver already walks the AST with scoped environments in `src/resolver.rs`. That is a good base.

Typechecker Zoo is the best next resource for implementation. It is practical Rust code for Algorithm W, bidirectional checking, row polymorphism, effects, etc. It is implementation-focused rather than the formal source of truth.

Appel is good, but not as your main type-inference resource. Use it for compiler architecture, environments, semantic analysis, IR, and "how a compiler is organized." For Hindley-Milner-style inference, Typechecker Zoo will be more directly useful.

## Recommended Route

1. Add a simple static type pass to `jlox-rs`.

   Start with:

   ```rust
   enum Type {
       Number,
       String,
       Bool,
       Nil,
       Function(Vec<Type>, Box<Type>),
       Any,
       Error,
   }
   ```

   Implement `typecheck_stmt` and `typecheck_expr` next to your resolver. Check literals, unary ops, binary ops, variables, assignment, calls, `if`, `while`, and returns. Do not implement inference variables yet.

2. Add explicit annotations.

   Extend Lox syntax gradually:

   ```lox
   var x: number = 1;

   fun add(a: number, b: number): number {
     return a + b;
   }
   ```

   This teaches parsing type syntax, storing declarations, checking assignability, function signatures, and return checking.

3. Then implement local inference.

   Infer:

   ```lox
   var x = 1;        // number
   var s = "hi";     // string
   var b = x > 2;    // bool
   ```

   At this point, read Typechecker Zoo's Algorithm W chapter. But be careful: full HM inference assumes mostly immutable, expression-oriented languages. Lox has mutation, statements, nil, runtime truthiness, and functions, so copy the concepts, not the whole design blindly.

4. Add constraint solving.

   Introduce type variables for unknowns:

   ```lox
   fun id(x) {
     return x;
   }
   ```

   Then generate constraints from usage. For example, `x + 1` constrains `x` to `number`; calling `f(1)` constrains one parameter of `f`.

5. Add custom types.

   Do this after basic inference works. Start with aliases and records:

   ```lox
   type User = { name: string, age: number };
   var u: User = { name: "Ada", age: 36 };
   ```

   Then add tagged unions/enums. Only after that should you study row polymorphism from Typechecker Zoo.

6. Study `new-nu-parser`.

   This is closer to your final destination than Nushell proper. It uses a type arena, `TypeId`s, side tables for node types, bidirectional checking, subtype constraints, `OneOf`/`AllOf`, and mutable lower/upper bounds for inference variables. That is more practical for a shell language than pure Algorithm W.

7. Study Nushell proper last.

   Nushell's public docs define user-facing data types like `int`, `float`, `string`, `bool`, `record`, `table`, `closure`, `nothing`, and `any`. Custom commands support parameter annotations and pipeline signatures like `string -> record`; Nushell says those are checked by the parser. Its production implementation separates runtime `Value`, static `Type`, parser `SyntaxShape`, and command `Signature`. That architecture is the main lesson.

## Suggested Reading Order

1. Crafting Interpreters chapter 11 recap: semantic passes and resolver.
2. Appel Chapter 5: Semantic Analysis.
3. Implement a simple explicit checker in `jlox-rs`.
4. Typechecker Zoo: Algorithm W.
5. Add inference variables/unification to your checker.
6. Appel Chapter 16: Polymorphic Types.
7. Typechecker Zoo advanced chapters: bidirectional checking, subtyping, row polymorphism.
8. `new-nu-parser` typechecker.
9. Nushell proper.

## Appel Chapters

Read Appel Chapter 5 before Typechecker Zoo.

Focus on:

- symbol tables
- environments
- bindings
- type-checking expressions
- type-checking declarations

This directly matches what your `resolver.rs` is already doing in `jlox-rs`.

Do not read all of Appel before Typechecker Zoo. Use Appel to understand compiler-pass architecture; use Typechecker Zoo for the inference machinery.

Optional Appel chapters:

- Chapter 4: Abstract Syntax, if you want cleaner AST visitor/pass structure.
- Chapter 14: Object-Oriented Languages, later if you continue Lox classes.
- Chapter 15: Functional Programming Languages, later for closures and function representation.
- Chapter 16: Polymorphic Types, after you have seen Algorithm W from Typechecker Zoo.

## Big Design Lesson From Nushell

Do not merge everything into one concept. Keep these separate:

```text
runtime Value
static Type
parser SyntaxShape / grammar expectation
function or command Signature
```

Sources:

- https://www.nushell.sh/book/types_of_data.html
- https://www.nushell.sh/book/custom_commands.html
- https://sdiehl.github.io/typechecker-zoo/
- https://www.cs.princeton.edu/~appel/modern/toc.html

# GPT-5.6 Terra
**Yes—those are good resources, and your plan is well sequenced.** Since you have completed *Crafting Interpreters* chapter 11, your interpreter already has the key prerequisite for a type checker: a separate lexical-resolution pass. Your `Resolver` maintains nested scopes and resolves variables before evaluation, so a type-inference pass can follow the same broad structure while producing types instead of scope depths.

Your current code is a good fit for this experiment: the AST already distinguishes expressions and statements, supports variables, functions, calls, blocks, conditionals, loops, and returns, and evaluation is cleanly separated from parsing. Start with a **new static analysis pass**, not by mixing types into runtime `Value`.

## Resource assessment

### 1. *Modern Compiler Implementation in Java* — good, with a caveat

It is strong for learning **compiler structure**, semantic analysis, environments/symbol tables, type representations, and error reporting. Its Java implementation language does not matter much—the concepts transfer directly to Rust.

However, don’t treat it as your sole guide for Hindley–Milner inference. Appel’s book is broader compiler education; it is excellent for understanding the role and design of a semantic-analysis/type-checking phase, but it is not the most incremental “implement Algorithm W from zero” tutorial.

**Use it for:**

- AST and semantic-analysis architecture
- Symbol/type environments
- Named types, records, aliases, functions, and type equivalence
- Designing diagnostics and a compiler pass pipeline

### 2. *Typechecker Zoo* — especially good for your goal

This should be your **implementation-first resource**. It walks through progressively richer type systems and gives you working algorithms to study. Although it uses Haskell, the core ideas map naturally to Rust:

- algebraic data types → Rust `enum`
- type environments → `HashMap<String, Scheme>`
- substitutions → `HashMap<TypeVarId, Type>`
- recursive traversal → `match` over your `Expr` / `Stmt`
- error propagation → `Result<T, TypeError>`

The biggest risk is copying a mature Hindley–Milner implementation before understanding each invariant. Implement each stage in a deliberately smaller language first.

## Recommended learning route

### Phase 0 — Preserve an untyped Lox baseline

Before changing syntax, make sure you have:

- regression tests for the programs you already support;
- a way to run “type checking succeeds/fails” tests;
- a clear pipeline:

```text
scan → parse → resolve → type-check → interpret
```

Keep `Resolver` and `TypeChecker` separate at first. Resolution answers **“which binding does this name refer to?”**; type checking answers **“what type does that binding/expression have?”**

---

### Phase 1 — Build a tiny expression type checker

Do **not** begin with full Lox, functions, or inference.

Create an internal type representation roughly like:

```rust
enum Type {
    Number,
    Bool,
    String,
    Nil,
    Unknown, // temporary teaching aid; remove or minimize later
}
```

Then type-check only:

- literals;
- unary `-` and `!`;
- arithmetic operators;
- comparisons;
- equality;
- logical `and` / `or`.

Examples:

```lox
print 1 + 2;        // number
print "a" + "b";    // string, if you retain Lox concatenation
print -true;        // static error
print 1 < false;    // static error
```

**Goal:** learn operator typing and source-located diagnostics, such as:

```text
[line 3] Type error: operator '+' requires two numbers or two strings;
found number and bool.
```

At this point, use direct checks, not unification.

---

### Phase 2 — Add annotated local variables and functions

Extend your Lox syntax with **explicit annotations** before trying inference. For example:

```lox
var count: number = 0;
var enabled: bool = true;

fun add(left: number, right: number): number {
  return left + right;
}
```

Add types for function signatures:

```rust
Type::Function(Vec<Type>, Box<Type>)
```

Then implement:

- annotated `var` declarations;
- parameter types;
- return types;
- function call checking;
- assignment checking;
- `if` / `while` condition must be `bool`;
- return-value validation.

This is the phase where Appel is particularly useful: environments, function signatures, checking declarations, and designing type-error messages.

**Important design choice:** decide whether an `if` expression exists. In current Lox it is a statement, so both branches can simply be checked independently. If you later add expression-valued `if`, you will need branch-type compatibility rules.

---

### Phase 3 — Replace annotations with monomorphic inference

Once annotations work, add a type-variable form:

```rust
Type::Var(TypeVarId)
```

For an unannotated variable:

```lox
var x = 42;
```

infer `x: number`.

For a function:

```lox
fun identity(x) {
  return x;
}
```

create fresh unknown types:

```text
x : α
return type : β
```

and constrain the body so the return expression has type `β`.

At this point, write a **unifier**. It is the central skill to learn:

```text
unify(number, number) = success
unify(α, number) = α := number
unify((α -> α), (number -> β)) = α := number, β := number
unify(number, bool) = error
```

Implement and test these pieces independently:

1. fresh type-variable allocation;
2. substitutions;
3. applying a substitution to a type;
4. unification;
5. the occurs check (`α` must not become `α -> number`).

Do not skip the occurs check merely because it is inconvenient—it teaches why infinite types are invalid.

---

### Phase 4 — Infer functions, but keep them monomorphic

Before introducing full Hindley–Milner polymorphism, let every binding receive one fixed inferred type.

For example:

```lox
fun double(x) {
  return x + x;
}
```

should infer:

```text
double : number -> number
```

But this function:

```lox
fun identity(x) {
  return x;
}
```

may initially infer only a single type based on its first usage. That limitation is expected in the monomorphic phase.

Test recursion explicitly:

```lox
fun factorial(n) {
  if (n <= 1) return 1;
  return n * factorial(n - 1);
}
```

For recursive functions, put a provisional fresh function type into the environment **before** checking the body, then unify it with the inferred signature afterward.

---

### Phase 5 — Implement Hindley–Milner let-polymorphism

This is where *Typechecker Zoo* becomes the primary guide.

Add type schemes:

```rust
struct Scheme {
    quantified: Vec<TypeVarId>,
    ty: Type,
}
```

Then implement:

- free type-variable calculation;
- environment free-variable calculation;
- **generalization** at variable bindings;
- **instantiation** at every variable use;
- Algorithm W, or an equivalent constraint-generation plus unification design.

Your key success test is:

```lox
fun identity(x) {
  return x;
}

print identity(1);
print identity(true);
```

The desired inferred type is:

```text
identity : forall a. a -> a
```

Each use of `identity` must get fresh type variables, allowing it to be called at both `number` and `bool`.

**Recommendation:** use `var` as your “let-like” generalization point initially. Be careful once you introduce mutation/assignment: unrestricted polymorphism plus mutable bindings has important soundness issues. For a learning interpreter, one practical route is:

- generalize only immutable `let`-style declarations that you add;
- keep mutable `var` bindings monomorphic;
- or temporarily disallow assignment while studying HM.

That is a valuable lesson, not an implementation nuisance.

---

### Phase 6 — Add custom type definitions

Only after inference is reliable should you add user-defined types. A sensible order is:

1. **Type aliases**
   ```lox
   type UserId = number;
   ```

2. **Records / structural data**
   ```lox
   type User = { name: string, age: number };
   ```

3. **Nominal records or classes**
   ```lox
   class User {
     init(name, age) { ... }
   }
   ```

4. **Algebraic data types / variants**
   ```lox
   type Option<T> =
     | Some(T)
     | None;
   ```

5. Pattern matching, if you want to explore the full benefit of ADTs.

For learning type-system fundamentals, ADTs plus pattern matching are more instructive than trying to statically type all of object-oriented Lox immediately. Classes, inheritance, `this`, and `super` add separate questions about nominal subtyping, method lookup, constructor typing, and possibly generics.

## Suggested repository architecture

Add a module rather than modifying `eval.rs` first:

```text
src/
  expr.rs
  stmt.rs
  resolver.rs
  types.rs        # Type, Scheme, type variables, substitutions
  typecheck.rs    # inference/checking pass and diagnostics
  eval.rs
```

A clean public interface could look like:

```rust
pub struct TypeChecker {
    // type environment, substitutions, fresh-variable counter, errors
}

impl TypeChecker {
    pub fn check(&mut self, statements: &[Stmt]) -> Result<(), Vec<TypeError>>;
}
```

Then invoke it after name resolution and before `Interpreter::interpret()`.

This also keeps a useful teaching distinction:

- `Value` in `eval.rs` = runtime values;
- `Type` in `types.rs` = compile-time/static descriptions;
- `Resolver` = lexical binding analysis;
- `TypeChecker` = static constraints and type errors.

## How this prepares you for Nushell

This is a strong path toward Nushell work, but don’t expect a textbook HM checker to drop directly into it.

Nushell is strongly and **gradually typed**: it supports annotations, compound types such as `list<string>` and record types, and an `any`-like escape hatch. Its type checking is intertwined with parsing and command signatures. ([nushell.sh](https://www.nushell.sh/lang-guide/chapters/types/type_signatures.html?utm_source=openai)) Recent Nushell work also includes parse-time propagation of command input/output types, union-like `oneof` behavior, and deliberately deferring uncertain cases to runtime—design concerns beyond classic Hindley–Milner inference. ([nushell.sh](https://www.nushell.sh/blog/2026-07-04-nushell_v0_114_0.html?utm_source=openai))

So the transferable skills you should target are:

- scoped type environments;
- unification and substitutions;
- type-variable generalization and instantiation;
- gradual-typing boundaries (`any`, unknown, dynamic values);
- union/optional types;
- record and row-like data typing;
- source-span-aware diagnostics;
- type flow through pipelines and function/command signatures.

That is a particularly relevant direction because Nu command parameters and pipeline input/output signatures carry types and can be checked during parsing. ([nushell.sh](https://www.nushell.sh/lang-guide/chapters/types/type_signatures.html?utm_source=openai))

## Short reading order

1. Finish your chapter-11 interpreter cleanup and tests.
2. Read the semantic-analysis/type-checking material in Appel.
3. Implement a tiny **annotated**, monomorphic checker in `jlox-rs`.
4. Work through the early *Typechecker Zoo* implementations until substitution and unification feel routine.
5. Implement monomorphic inference in your own AST.
6. Implement HM let-polymorphism with schemes.
7. Study gradual typing, unions, records, and command/pipeline typing before attempting Nushell-facing contributions.
8. Read the relevant Nushell parser/type-checking code only after you can explain your own checker’s invariants.

**My strongest recommendation:** make `jlox-rs` intentionally become a small typed language rather than trying to “bolt full inference onto Lox” in one change. Build annotations → monomorphic checking → unification → inference → polymorphism → custom types. Each layer gives you a runnable checkpoint and maps directly to a real compiler concept.

# Gemini
Yes, Appel's "Tiger Book" and Stephen Diehl's "[Typechecker Zoo](https://www.google.com/search?q=Typechecker+Zoo&kgmid=/hkb/Cg4KCGxhbmd1YWdlEgJlbgoMCgR0eXBlEgRCT09LCh4KC2VudGl0eV9uYW1lEg90eXBlY2hlY2tlciB6b28%3D)" are excellent, highly respected choices for learning type systems and type inference.
However, because you are moving from a dynamically typed interpreter (jlox) toward adding a static type inference system, and your ultimate goal is contributing to Nushell (a language with a unique, structural, pipe-oriented type system), you need a very deliberate roadmap. Nushell relies heavily on row polymorphism, structural types, and flow-based data, which are much more advanced than standard object-oriented typing.
Here is an analysis of your resources and a custom learning route designed to get you from Chapter 11 of jlox to hacking on Nushell's parser.
------------------------------
## Resource Evaluation

* 
* Modern Compiler Implementation in Java (Appel):
* The Good: Chapters 4 and 5 provide an excellent, rigorous introduction to abstract syntax, symbol tables, and type checking for an imperative, statically typed language (Tiger).
   * The Caveat: It focuses mostly on explicitly typed checking (like Java/C) rather than implicit Type Inference (like Hindley-Milner). It will give you a rock-solid foundation for basic type rules, but won't fully solve your inference goals.
* Typechecker Zoo:
* The Good: Phenomenal for Type Inference. It takes you step-by-step from simple type checkers to Hindley-Milner (HM), Bidirectional typing, and Subtyping.
   * The Caveat: The code implementations are in Haskell. Since your repo is in Rust (jlox-rs), you will need to translate functional paradigms (like monads and immutable maps) into Rust idioms (like Rc, RefCell, or passing mutable context state).
* 

------------------------------
## Recommended 4-Step Learning Route
To bridge the gap between jlox-rs and nushell, follow this sequential route:
## 1. Formulate the Basics (Appel Chapter 4 & 5)
Before inferring types, you must know how to represent them in memory.

* 
* Goal: Read Appel's chapters on Type Checking.
* Action: Define a Type enum in Rust for your current Jlox (e.g., Type::Number, Type::String, Type::Boolean, Type::Nil, Type::Function). Write a basic, explicit checker that rejects code like "hello" + 5 before execution.
* 

## 2. Master Hindley-Milner (Typechecker Zoo: Poly & Ido)
This is where you learn true type inference (the math behind languages like Rust, Haskell, and OCaml).

* 
* Goal: Learn Algorithm W (unification and substitution).
* Action: Study the Poly mini-language in Typechecker Zoo. Implement a tiny, standalone Hindley-Milner inference engine in Rust. You will learn how to generate type variables (T₁, T₂), unify them ($T_1 \equiv \text{String}$), and handle polymorphism.
* 

## 3. Retrofit jlox-rs with Type Inference
Apply your standalone engine directly back to your current codebase.

* 
* Goal: Turn Jlox into a statically inferred language.
* Action: Add a Type Checker pass between your Resolver (Chapter 11) and your Interpreter (Chapter 7). Instead of executing code, this pass walks the AST, assigns type variables to expressions, unifies them, and decorates the AST with concrete types.
* 

## 4. Study Nushell's Type System & Architecture
Nushell does not use pure Hindley-Milner because it deals with command-line pipelines and tables.

* 
* Goal: Understand how Nushell tracks types through pipelines.
* Action: Read through Nushell's Type Source Code. Nushell uses a blend of explicit types, structural record types (checking if a table has a specific column), and signature matching for commands.
* 

------------------------------
## Strategic Comparison: HM vs. Nushell Type System

| Feature | Standard HM (Typechecker Zoo) | Nushell Type System |
|---|---|---|
| Primary Goal | Infer generic functional types | Validate table schemas and pipeline inputs |
| Core Mechanism | Global unification (X = Y) | Signature matching & Flow checking |
| Complex Case | Higher-order functions | Records, Tables, and Custom Blocks |

------------------------------
## Structural Advice for Rust Implementation
Since you are implementing this in Rust, avoid deeply nested pointer-based graph unification initially. It triggers Rust's borrow checker nightmares. Instead, use an Arena or an Index-based Substitution map (using Vec indices instead of raw references) to keep your type inference engine clean and memory-safe.
To help tailor this, how far do you want to push type inference in Jlox? Are you looking to keep it simple (primitives and functions), or do you want to infer complex types like classes and instances?


# Summary

All three outputs agree on the core path:

- Use Appel for compiler-pass structure: ASTs, environments, symbol tables, semantic analysis.
- Use Typechecker Zoo for real inference machinery: type variables, substitutions, unification, Algorithm W, polymorphism.
- Use jlox-rs as the learning playground before trying to affect Nushell.
- Study new-nu-parser before Nushell proper because it is more focused on modern type-checking architecture.
- Do not jump directly to custom types, row polymorphism, or Nushell pipeline typing.

The best single recommendation is from the Terra output: build in layers, with runnable checkpoints.

Best Order

1. Clean up current jlox-rs baseline
  - Make sure scan -> parse -> resolve -> interpret still works.
  - Add a few regression tests or sample programs.
  - Keep runtime Value and static Type separate.

2. Read Appel Chapter 5: Semantic Analysis
  - Focus on symbol tables, environments, declarations, expression checking.
  - Skim Chapter 4 only if you want better AST/pass structure.
  - Do not read all of Appel first.

3. Implement a tiny explicit type checker (Skip, because jlox doesn't use type directly)
  - Add src/types.rs and src/typecheck.rs.
  - Start with:

    enum Type {
        Number,
        String,
        Bool,
        Nil,
        Function(Vec<Type>, Box<Type>),
        Error,
    }

  - Check literals, unary ops, binary ops, if, while, variables, assignment, calls, and returns.
  - No inference variables yet.

4. Add explicit annotations (Skip for now, because it requires more syntax changes)

 var x: number = 1;

 fun add(a: number, b: number): number {
   return a + b;
 }

 This teaches the most important practical pieces: parsing type syntax, storing declared types, checking assignments, function signatures, and return types.

5. Read Typechecker Zoo: substitutions, unification, Algorithm W (Algorithm W chapter)
  - Learn type variables.
  - Learn substitutions.
  - Learn occurs check.
  - Learn unification.
  - A tiny standalone Rust experiment is useful here, but do not spend too long on it.

6. Add monomorphic inference to jlox-rs

 var x = 1;      // number
 var s = "hi";   // string
 var b = x > 2;  // bool

 Then infer simple function types, but keep each binding one fixed type at first.

7. Handle functions and recursion
  - Give recursive functions provisional fresh types before checking their bodies.
  - Then unify the inferred body/return type afterward.

8. Implement HM let-polymorphism
  - Add Scheme.
  - Add generalization.
  - Add instantiation.
  - Be careful with mutation: either generalize only immutable bindings or keep mutable var monomorphic.

9. Add custom types
 Recommended order:
  - type aliases
  - records
  - nominal records/classes
  - tagged unions / ADTs
  - pattern matching

10. Study Typechecker Zoo advanced topics

- bidirectional checking
- subtyping
- row polymorphism
- records
- effects, only later

11. Study new-nu-parser

- Type arena / TypeId
- side-table node typing
- bidirectional checking
- lower/upper bounds for inference variables
- OneOf / AllOf
- gradual typing ideas

12. Study Nushell proper

  - Value
  - Type
  - SyntaxShape
  - command Signature
  - pipeline input/output typing
  - any as an escape hatch

  The important sequence is:

  Appel Ch. 5
  -> explicit checker
  -> Typechecker Zoo Algorithm W
  -> monomorphic inference
  -> HM polymorphism
  -> custom types
  -> bidirectional/subtyping/records
  -> new-nu-parser
  -> Nushell

  I would not put standalone HM before the explicit jlox-rs checker. The explicit checker gives you compiler-pass intuition first; then Typechecker Zoo will make more sense.

# Typechecker zoo actual read
For your goal, the core path is:

1. Simply Typed Lambda Calculus
  - Basic type rules.
  - Good warmup for expr -> type.

2. Hindley-Milner
  - Main chapter for your current goal.
  - Learn type variables, substitutions, unification, generalization, instantiation.

3. Bidirectional Type Checking
  - Important for understanding new-nu-parser.
  - Useful once explicit annotations and inference interact.

4. Subtyping
  - Useful before Nushell-style oneof, any, records, and gradual boundaries.

5. Row Polymorphism
  - Read when you start records/tables/custom structural types.
  - Especially relevant to Nushell.

Optional later:

- System F: useful background for generics, but not required early.
- Type Classes: useful if you want ad-hoc overloading like + for number/string/list.
- Effects: interesting, but not needed for your current direction.
- Dependent Types / Linear Types / advanced chapters: skip for now.

Best practical order:

Simply Typed Lambda Calculus
    -> Hindley-Milner
    -> implement inference in jlox-rs
    -> Bidirectional Type Checking
    -> Subtyping
    -> Row Polymorphism

## Actual chapters
I think you mean actual chapters/pages in Typechecker Zoo. Based on its current SUMMARY.md, use this order:

1. Foundations
  - Foundations -> Type Systems
  - https://sdiehl.github.io/typechecker-zoo/foundations/type-systems.html

2. Algorithm W
  - Algorithm W -> Lambda Calculus
  - https://sdiehl.github.io/typechecker-zoo/algorithm-w/lambda-calculus.html
  - Algorithm W -> Type Inference
  - https://sdiehl.github.io/typechecker-zoo/algorithm-w/implementation.html
  - Algorithm W -> Examples
  - https://sdiehl.github.io/typechecker-zoo/algorithm-w/examples.html

3. System F
  - System F -> System F
  - https://sdiehl.github.io/typechecker-zoo/system-f/system-f.html
  - System F -> Type Checker
  - https://sdiehl.github.io/typechecker-zoo/system-f/implementation.html
  - System F -> Examples
  - https://sdiehl.github.io/typechecker-zoo/system-f/examples.html

4. Row Polymorphism
  - Row Polymorphism -> Overview
  - https://sdiehl.github.io/typechecker-zoo/row-poly/overview.html
  - Row Polymorphism -> Implementation
  - https://sdiehl.github.io/typechecker-zoo/row-poly/implementation.html
  - Row Polymorphism -> Examples
  - https://sdiehl.github.io/typechecker-zoo/row-poly/examples.html

Optional later:

5. Type Classes
  - Read if you want principled overloading, such as + working for numbers, strings, lists, etc.
  - https://sdiehl.github.io/typechecker-zoo/type-classes/overview.html

Skip for now:

- System Fω
- Refinement Types
- Calculus of Constructions
- Row Effects
- Call-by-Push-Value

So the practical order is:

Foundations / Type Systems
    -> Algorithm W / Lambda Calculus
    -> Algorithm W / Type Inference
    -> Algorithm W / Examples
    -> implement inference in jlox-rs
    -> System F / Type Checker
    -> Row Polymorphism
    -> new-nu-parser
    -> Nushell

Correction to my earlier wording: Typechecker Zoo does not have a top-level chapter literally named “Simply Typed Lambda Calculus” or “Subtyping.” The actual relevant pages are Foundations -> Type Systems,
Algorithm W -> Lambda Calculus, System F -> Type Checker, and Row Polymorphism.

Source: Typechecker Zoo lists its main systems as Algorithm W, Type Classes, System F, System Fω, Refinement Types, Calculus of Constructions, Row Polymorphism, Row Effects, and Call-by-Push-Value. Its
SUMMARY.md gives the concrete page structure.
