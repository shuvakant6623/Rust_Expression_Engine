🧠 Rust Expression Engine

A compiler-inspired expression engine implemented entirely from scratch in Rust, designed to parse, transform, and deterministically evaluate human-written mathematical expressions.

The project follows a real compiler pipeline (Lexer → Parser → IR → Evaluator) and focuses on systems design, execution semantics, and correctness, not shortcuts or library abstractions.

Status: Core compiler pipeline complete
(Lexer → Parser → Postfix IR → Evaluator → CLI)

🎯 Motivation

Modern systems such as compilers, query engines, and feature stores never execute raw strings directly.

Instead, they:

Tokenize input

Resolve syntax and operator precedence

Make execution order explicit

Evaluate deterministically

This project recreates that pipeline from first principles to deeply understand:

how interpreters are built

how execution order is derived

how systems separate parsing, semantics, and execution

✨ Features

Custom Lexer (iterator-based, from scratch)

Shunting Yard Parser (infix → postfix)

Explicit Postfix Intermediate Representation (IR)

Stack-based Evaluator / Interpreter

Variable assignments via symbol table

Interactive CLI

Clean, compiler-style modular architecture

No eval, no parsing libraries, no shortcuts

🧩 Example
Input
a = 10
b = 5
(a + b) * 3

Output
Result: 45

🏗️ Architecture Overview
Raw Expression (text)
        ↓
Lexical Analysis (Tokenizer)
        ↓
Syntax Analysis (Parser)
        ↓
Postfix IR (Explicit Execution Order)
        ↓
Evaluation (Interpreter)


Each phase is:

isolated

testable

replaceable

This mirrors real compiler and query-engine pipelines.

📂 Project Structure
expression_engine/
│
├── Cargo.toml
├── README.md
│
└── src/
    ├── main.rs              # CLI driver
    │
    ├── core/                # Language primitives
    │   ├── token.rs
    │   ├── stack.rs
    │   └── precedence.rs
    │
    ├── lexer/               # Lexical analysis
    │   └── tokenizer.rs
    │
    ├── parser/              # Syntax analysis
    │   └── infix_to_postfix.rs
    │
    ├── context/             # Symbol table
    │   └── context.rs
    │
    └── eval/                # Execution engine
        └── evaluator.rs


The structure intentionally mirrors production compiler layouts.

🧠 Key Concepts Demonstrated

Language modeling using Rust enums

Generic data structures (Stack<T>)

Ownership & borrowing for memory safety

Iterator-based lexical analysis

Stack-based parsing and execution

Deterministic execution model

Clear separation of compiler phases

🛠️ How to Run
cargo run

CLI Commands

Variable assignment

x = 10


Expression evaluation

(x + 5) * 2


Exit

exit

🚧 Current Limitations (Intentional)

Integer-only arithmetic

No floating-point support

No functions yet (sin, log, etc.)

No AST or symbolic calculus

Minimal error handling

These are deliberate design boundaries, not gaps in understanding.

🔮 Planned Extensions

Abstract Syntax Tree (AST)

Symbolic differentiation (chain & product rule)

Limited symbolic integration

Expression simplification

Compiler-style optimizations

Test suite & benchmarks

📚 Learning Outcomes

This project emphasizes systems thinking over math tricks.

It demonstrates:

how interpreters and compilers are structured

how execution order is derived from syntax

how deterministic, safe execution engines are designed in Rust