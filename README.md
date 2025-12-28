## 🧠 Rust Expression Engine

A **compiler-inspired expression engine** written entirely from scratch in **Rust**.

This project parses, transforms, and deterministically evaluates **human-written mathematical expressions** by following a real **compiler pipeline** rather than executing raw strings.

---

## 🎯 Motivation

Modern systems (compilers, query engines, feature stores) never evaluate raw text directly.

Instead, they:

- Tokenize input
- Resolve syntax and operator precedence
- Make execution order explicit
- Evaluate deterministically

This project recreates that pipeline from first principles to deeply understand:

- how interpreters are built  
- how execution order is derived  
- how systems separate parsing, semantics, and execution  

---

## ✨ Features

- Custom **Lexer** (iterator-based, from scratch)
- **Shunting Yard Parser** (infix → postfix)
- Explicit **Postfix Intermediate Representation (IR)**
- Stack-based **Evaluator / Interpreter**
- Variable assignments via **symbol table**
- Interactive **CLI**
- Clean, compiler-style modular architecture
- No `eval`, no parsing libraries, no shortcuts

---

## 🧩 Example

### Input

a = 10
b = 5
(a + b) * 3


### Output

Result: 45


---

## 🏗️ Architecture Overview

Raw Expression (text)
↓
Lexical Analysis (Tokenizer)
↓
Syntax Analysis (Parser)
↓
Postfix IR (Explicit Execution Order)
↓
Evaluation (Interpreter)


Each phase is **isolated and testable**.

---

## 📂 Project Structure

expression_engine/
│
├── Cargo.toml
├── README.md
│
└── src/
├── main.rs # CLI driver
│
├── core/ # Language primitives
│ ├── token.rs
│ ├── stack.rs
│ └── precedence.rs
│
├── lexer/ # Lexical analysis
│ └── tokenizer.rs
│
├── parser/ # Syntax analysis
│ └── infix_to_postfix.rs
│
├── context/ # Symbol table
│ └── context.rs
│
└── eval/ # Execution engine
└── evaluator.rs


This structure mirrors **real compiler pipelines**.

---

## 🧠 Key Concepts Demonstrated

- Enums for language modeling
- Generic data structures (`Stack<T>`)
- Ownership & borrowing for memory safety
- Iterator-based lexical analysis
- Stack-based parsing and execution
- Deterministic execution model
- Clear separation of compiler phases

---

## 🛠️ How to Run

```bash
cargo run

CLI Commands

Variable assignment

x = 10

Expression evaluation

(x + 5) * 2

Exit

exit

---

## 🚧 Current Limitations (Intentional)

    Integer-only arithmetic

    No floating-point support

    No functions yet (sin, log, etc.)

    No AST or symbolic calculus

    Minimal error handling

These are deliberate design decisions, not missing understanding.
🔮 Planned Extensions

    Abstract Syntax Tree (AST)

    Symbolic differentiation (chain & product rule)

    Limited symbolic integration

    Expression simplification

    Compiler-style optimizations

    Test suite & benchmarks

📚 Learning Outcome

This project focuses on systems thinking, not math tricks.

It demonstrates:

    how interpreters and compilers are structured

    how execution order is derived from syntax

    how deterministic execution engines are designed in Rust