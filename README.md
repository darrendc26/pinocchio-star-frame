# Star Frame

<p align="center">
  <strong>A high-performance Solana framework for building fast, scalable, and secure smart contracts</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/star_frame"><img src="https://img.shields.io/crates/v/star_frame?logo=rust" /></a>
  <a href="https://docs.rs/star_frame"><img src="https://img.shields.io/docsrs/star_frame?logo=docsdotrs" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue" /></a>
</p>

## 🌟 What is Star Frame?

**Star Frame** is a trait-based framework built on top of [Pinocchio](https://github.com/febo/pinocchio) for developing Solana programs (smart contracts) with maximum performance and type safety. It provides a comprehensive toolkit that makes it easier to write efficient, reliable, and maintainable Solana programs while maintaining the low-level performance characteristics critical for blockchain applications.

### Key Features

- **🚀 High Performance**: Built on Pinocchio, which uses zero-copy deserialization and minimal allocations for maximum efficiency
- **🛡️ Type Safety**: Leverages Rust's powerful type system with trait-based abstractions
- **📦 Zero-Copy Serialization**: Uses `bytemuck` for Pod types, eliminating serialization overhead
- **🧩 Modular Design**: Organized into focused crates for different aspects of program development
- **🔧 Developer Tools**: CLI tool for scaffolding new projects with best practices
- **📋 IDL Generation**: Automatic Interface Definition Language (IDL) generation for client integration
- **✅ Account Validation**: Comprehensive account validation system with seed derivation support

## 📚 Architecture

This repository is organized as a Rust workspace containing several interconnected crates:

### Core Crates

#### `star_frame` - The Main Framework
The primary framework providing core traits and abstractions for building Solana programs:
- **Instruction Processing**: `InstructionSet`, `StarFrameInstruction` traits for handling program instructions
- **Account Management**: `AccountSet` for grouped account validation and lifecycle management
- **Context System**: Execution context with cached sysvars (clock, rent) and rent management
- **Error Handling**: Comprehensive error system with custom error code support

#### `star_frame_proc` - Procedural Macros
Derive macros and procedural macros that power Star Frame's ergonomic API:
- `#[derive(StarFrameProgram)]` - Define your program with entrypoint generation
- `#[derive(InstructionSet)]` - Create instruction enums with automatic dispatching
- `#[derive(AccountSet)]` - Define account groups with validation and lifecycle management
- `#[star_frame_instruction]` - Annotate instruction handler functions
- `#[zero_copy]` - Create zero-copy account data structures
- And many more for seeds, errors, and IDL generation

#### `star_frame_idl` - IDL Generation
IDL (Interface Definition Language) generation and management:
- Automatic IDL generation from program code
- Codama integration for client SDK generation
- Type definitions for accounts, instructions, and errors
- Metadata and versioning support

#### `star_frame_spl` - SPL Token Integration
Integration helpers for Solana Program Library (SPL) tokens:
- Associated Token Account utilities
- Token program interactions
- Pod-compatible SPL types

#### `star_frame_cli` - Command Line Tool
Developer tool for project scaffolding:
- `sf new <project-name>` - Create new Star Frame projects
- Pre-configured templates with best practices
- Example counter program to get started quickly

## 🚀 Quick Start

### Installation

Install the Star Frame CLI:

```bash
cargo install star_frame_cli
```

### Create a New Project

```bash
sf new my_program
cd my_program
```

This generates a complete Solana program with:
- A working counter example program
- Account state definitions
- Instruction handlers
- Unit tests with Mollusk SVM
- Proper project structure

### Project Structure

```
my_program/
├── src/
│   ├── lib.rs              # Program definition and instruction set
│   ├── states.rs           # Account state structures
│   ├── instructions/       # Instruction implementations
│   │   ├── initialize.rs
│   │   ├── increment.rs
│   │   └── mod.rs
│   └── tests/              # Unit tests
│       ├── counter.rs
│       └── mod.rs
├── Cargo.toml
└── README.md
```

## 💡 Example Program

Here's a simplified example of what a Star Frame program looks like:

```rust
use star_frame::prelude::*;

// Define your program
#[derive(StarFrameProgram)]
#[program(
    instruction_set = MyInstructionSet,
    id = "YourProgramID111111111111111111111111111"
)]
pub struct MyProgram;

// Define your instruction set
#[derive(InstructionSet)]
pub enum MyInstructionSet {
    Initialize(InitializeInstruction),
    Increment(IncrementInstruction),
}

// Define account state
#[zero_copy(pod)]
#[derive(ProgramAccount)]
pub struct CounterAccount {
    pub authority: Pubkey,
    pub count: u64,
}

// Define instruction handler
#[star_frame_instruction]
fn Initialize(accounts: &mut InitializeAccounts, start_at: Option<u64>) -> Result<()> {
    **accounts.counter.data_mut()? = CounterAccount {
        authority: *accounts.authority.pubkey(),
        count: start_at.unwrap_or(0),
    };
    Ok(())
}
```

## 🔍 Why Star Frame?

### Performance-First Design
Star Frame is built on Pinocchio, which provides the fastest Solana program runtime by:
- Using zero-copy deserialization
- Minimizing heap allocations
- Direct memory manipulation where safe
- Pod types via `bytemuck` for maximum efficiency

### Developer Experience
While maintaining performance, Star Frame adds:
- **Type-safe abstractions** that prevent common bugs
- **Clear separation of concerns** with AccountSets and Instructions
- **Comprehensive validation** built into the framework
- **Automatic IDL generation** for easy client integration
- **Excellent error messages** to debug issues quickly

### Production Ready
- Used in production Solana programs
- Maintained by Star Atlas Meta team
- Regular updates and improvements
- Comprehensive testing infrastructure

## 🔗 Relationship to Other Frameworks

### vs Anchor
- **Star Frame**: Zero-copy focused, maximum performance, Pinocchio-based
- **Anchor**: More batteries-included, easier for beginners, broader ecosystem

### vs Raw Solana SDK
- **Star Frame**: High-level abstractions while maintaining performance
- **Raw SDK**: Maximum control but more verbose and error-prone

### Built on Pinocchio
Star Frame extends [Pinocchio](https://github.com/febo/pinocchio) by adding:
- Trait-based instruction dispatch
- Account set validation framework
- Procedural macros for ergonomics
- IDL generation
- Project scaffolding tools

## 📖 Documentation

- [Crate Documentation](https://docs.rs/star_frame)
- [Star Frame IDL](https://docs.rs/star_frame_idl)
- [Star Frame CLI](https://docs.rs/star_frame_cli)
- [Pinocchio Documentation](https://docs.rs/pinocchio)

## 🛠️ Building and Testing

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Build for Solana BPF
cargo build-sbf

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## 🤝 Contributing

Contributions are welcome! This is a fork focused on maintaining and extending Star Frame functionality.

## 📄 License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on top of [Pinocchio](https://github.com/febo/pinocchio) by febo
- Originally developed by the Star Atlas Meta team
- Inspired by the Solana developer community

## 🔗 Related Projects

- [Pinocchio](https://github.com/febo/pinocchio) - The underlying zero-copy Solana framework
- [Anchor](https://github.com/coral-xyz/anchor) - Alternative Solana framework
- [Solana Program Library](https://github.com/solana-labs/solana-program-library) - Official Solana programs

---

<p align="center">
  Built with ❤️ for the Solana ecosystem
</p>
