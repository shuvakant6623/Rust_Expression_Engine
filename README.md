# Rust Expression Engine

A **compiler-inspired mathematical expression engine** written entirely from scratch in **Rust**.

This project parses, validates, and evaluates **human-written mathematical expressions** using a custom **lexer**, **parser**, and **stack-based evaluator**, following real **compiler and interpreter design principles**.

> ⚙️ Status: Core execution pipeline complete (Lexer → Parser → Evaluator)  
> 🧪 Tested and benchmarked using Rust’s standard tooling

---

## 🚀 Motivation

Modern systems (compilers, query engines, rule engines, feature stores) do **not execute raw strings directly**.

They:
1. Tokenize input
2. Apply grammar rules and precedence
3. Build an execution plan
4. Execute deterministically and safely

This project recreates that pipeline from scratch to deeply understand:
- how expressions are parsed
- how execution order is enforced
- how errors are modeled instead of panicking
- how Rust’s type system guarantees correctness

The goal is **learning systems-level engineering**, not building a calculator.

---

## 🧠 Architecture Overview
```bash
Input String
↓
Lexer (Tokenizer)
↓
Parser (Infix → Postfix, Shunting Yard)
↓
Evaluator (Stack-based execution)
↓
Result / Structured Error
```

Each stage is implemented as a **separate module**, mirroring real compiler architecture.

---

## ✨ Features

### Core Functionality
- Custom **lexer** for tokenizing numbers, variables, operators, and parentheses
- **Shunting Yard algorithm** for infix → postfix conversion
- **Stack-based evaluator** for deterministic execution
- Variable support via a **symbol table (Context)**
- Correct handling of:
  - operator precedence
  - parentheses
  - invalid expressions

### Safety & Correctness
- Typed error handling using `Result<T, E>`
- No `panic!`, no `unwrap()` in production code
- Zero `unsafe` Rust
- Invalid input never crashes the program

### Engineering Quality
- Clean **library + binary separation**
- Integration tests for parser and evaluator
- Benchmarks using **Criterion**
- Measured performance (~300 ns per evaluation on simple expressions)

---

## 📁 Project Structure
```bash
src/
├── lib.rs # Library crate (public API)
├── main.rs # CLI entry point
├── engine.rs # End-to-end evaluation pipeline
├── lexer/ # Tokenizer
├── parser/ # Infix → Postfix conversion
├── eval/ # Expression evaluator
├── context/ # Variable symbol table
├── core/ # Shared utilities (stack, precedence)
├── errors/ # Structured error types
tests/
├── lexer_tests.rs
├── parser_tests.rs
├── evaluator_tests.rs
benches/
└── expression_bench.rs # Criterion benchmarks
```

This layout follows **idiomatic Rust project conventions** used in production systems.

---

## 🧪 Testing

The project includes **integration tests** that validate:
- correct evaluation of valid expressions
- detection of invalid syntax (e.g. unmatched parentheses)
- safe failure on malformed input

Run all tests with:
```bash
cargo test
```
---

## ⚡ Benchmarking

Performance is measured using Criterion, Rust’s statistical benchmarking framework.

Run benchmarks with:
```bash
cargo bench
```
Example result:
```bash
evaluate (a+b)*3
time: [316 ns 318 ns 321 ns]
```
This includes tokenization, parsing, and evaluation.

---

## ▶️ Running the CLI
```bash
cargo run
```
Example:
```bash
> (a + b) * 3
Result: 45
```
Type exit to quit.

---

## 🧠 What This Project Demonstrates

    Understanding of compiler/interpreter pipelines

    Strong grasp of Rust ownership, borrowing, and enums

    Idiomatic error handling with Result

    Modular system design

    Testable and benchmarked code

    Comfort debugging real Rust tooling issues (Cargo, benches, modules)

---

## 📌 Future Work

    Build a full AST

    Add symbolic differentiation and simplification

    Add constant folding and optimizations

    Extend grammar (functions, unary ops)

    Explore JIT or bytecode-style execution

---

## 👤 Author

Shuvakant Patra
GitHub: https://github.com/shuvakant6623

---

## 📄 License

MIT License