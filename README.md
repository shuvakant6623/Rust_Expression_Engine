# 🧠 Rust Expression Engine

A **compiler-inspired expression engine** written entirely from scratch in **Rust**.

This project parses, transforms, and evaluates **human-written mathematical expressions** using a custom **lexer**, **parser**, and **stack-based evaluator**, following real **compiler design principles**.

> ⚙️ Status: Core compiler pipeline complete (Lexer → Parser → Evaluator → CLI)

---

## 🎯 Project Motivation

Modern systems (compilers, query engines, feature stores) do **not** execute raw strings directly.

They:
1. Tokenize input
2. Resolve syntax and precedence
3. Build an execution plan
4. Execute deterministically

This project recreates that pipeline in Rust to deeply understand:
- how expressions are interpreted
- how execution order is made explicit
- how systems separate logic from execution

---

## ✨ Features Implemented

✔ Custom **Lexer** (tokenizer)  
✔ **Shunting Yard Parser** (infix → postfix)  
✔ Explicit **postfix intermediate representation (IR)**  
✔ Stack-based **Evaluator / Interpreter**  
✔ **Variable support** via symbol table  
✔ Interactive **CLI interface**  
✔ Clean, compiler-style architecture  
✔ Written 100% from scratch (no eval hacks)

---

## 🧩 Example

### Input

a = 10
b = 5
(a + b) * 3


### Output

Result: 45


---

## 🏗️ Compiler-Style Architecture

Raw Expression (text)
↓
Lexical Analysis (Tokenizer)
↓
Syntax Analysis (Parser)
↓
Postfix IR (Execution Order)
↓
Evaluation (Interpreter)


Each phase is isolated and testable.

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
│ ├── mod.rs
│ ├── token.rs
│ ├── stack.rs
│ └── precedence.rs
│
├── lexer/ # Lexical analysis
│ ├── mod.rs
│ └── tokenizer.rs
│
├── parser/ # Syntax analysis
│ ├── mod.rs
│ └── infix_to_postfix.rs
│
├── context/ # Symbol table
│ ├── mod.rs
│ └── context.rs
│
└── eval/ # Execution engine
├── mod.rs
└── evaluator.rs


This structure mirrors **real compiler pipelines**.

---

## 🧠 Key Concepts Demonstrated

- Enums for language modeling
- Generic data structures (`Stack<T>`)
- Ownership & borrowing for memory safety
- Iterator-based lexical analysis
- Stack-based parsing & execution
- Deterministic execution model
- Clear separation of compiler phases

---

## 🛠️ How to Run

```bash
cargo run

CLI Commands

    Variable assignment:

x = 10

Expression evaluation:

(x + 5) * 2

Exit:

    exit

🚧 Current Limitations (Intentional)

    Integer-only arithmetic

    No floating point

    No functions yet (sin, log, etc.)

    No AST or symbolic calculus

    Minimal error handling

These are design decisions, not missing understanding.
🔮 Planned Extensions

After stabilizing the core engine:

    Abstract Syntax Tree (AST)

    Symbolic differentiation (chain & product rule)

    Limited symbolic integration

    Expression simplification

    Compiler-style optimizations

    Test suite & benchmarks

📚 Learning Outcome

This project focuses on systems thinking, not math tricks.

It demonstrates:

    how interpreters are built

    how compilers reason about execution

    how deterministic systems are designed