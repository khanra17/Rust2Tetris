# Rust2Tetris

## 🚀 **Building a Computer from Scratch Using Rust**

Welcome to **Rust2Tetris**, a journey where we build a computer from scratch using the Rust programming language! Inspired by the *[Nand2Tetris](https://www.nand2tetris.org/)* course, this project will take you from the basics of logic gates to designing a CPU, creating an assembler, building a compiler, and finally running a simple game of Tetris on our custom-built Rust-based computer.

> ⚠️ **Note**: This project is just getting started, and the structure will evolve as I progress through the course. Stay tuned!

---

## 📜 **Project Overview**

This project mirrors the *Nand2Tetris* curriculum, but with a Rust twist. Here's what we will build step-by-step:

1. **Basic Logic Gates**:  
   Simulating gates like `AND`, `OR`, `NOT`, etc., using Rust structs and basic logic.

2. **Complex Circuits**:  
   Using these gates to construct more advanced circuits like an *Arithmetic Logic Unit `(ALU)`* and memory systems like RAM.

3. **CPU**:  
   Designing a simple CPU capable of processing instructions.

4. **Assembler & Compiler**:  
   Building an assembler to convert assembly into machine code and a compiler for higher-level language constructs.

5. **Tetris**:  
   Finally, we'll load a Tetris game to run on the custom CPU.

---

## 🧠 **Core Concepts** *(Initial Plan)*

We will start by simulating basic logic gates. For example, an `AND` gate in Rust might look like this:

```rust
struct AndGate {
    a: bool,
    b: bool,
}

impl AndGate {
    fn output(&self) -> bool {
        self.a && self.b
    }
}
```

---

## 🏗️ **Project Structure**

The project is organized as follows:

```text
Rust2Tetris/
├── src/
│ ├── lib.rs
│ └── gates/
│  ├── mod.rs
│  ├── and.rs
│  ├── nand.rs
│  ├── not.rs
│  ├── ...
├── Cargo.toml
└── README.md
```

---

## 🧪 **Development Practices**

- **Test-Driven Development (TDD)**: We write tests before implementing the functionality.
- **Exhaustive Testing**: We aim for 100% test coverage using [`tarpaulin`](https://github.com/xd009642/tarpaulin).
- **Idiomatic Rust**: We follow Rust best practices and idioms.
- **Documentation**: All public items are documented using rustdoc comments.

To run tests and check coverage:

```bash
cargo test
cargo tarpaulin
```

---

## 🤝 Contributing

This project is in its early stages, and contributions are welcome! If you're interested in contributing or have suggestions, feel free to open an issue or submit a pull request.
