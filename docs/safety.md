# Safety Policy

## Overview

Chloe maintains a **zero-tolerance policy for unsafe code**. This document explains why and how we enforce memory and thread safety.

## Policy Statement

**All code in this project MUST be 100% safe Rust. No exceptions.**

This means:
- ❌ NO `unsafe` blocks
- ❌ NO `unsafe fn` definitions
- ❌ NO `unsafe impl`
- ❌ NO unsafe threading patterns
- ❌ NO raw pointer manipulation
- ❌ NO unchecked operations

## Enforcement

### Compiler-Level
```rust
// src/main.rs
#![forbid(unsafe_code)]
```

The `forbid` directive (stronger than `deny`) prevents ANY unsafe code, even if explicitly allowed elsewhere.

### Static Analysis
```toml
# Cargo.toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
undocumented_unsafe_blocks = "forbid"
magic_numbers = "warn"
```

These lints enforce:
- **No unsafe code** - Any `unsafe` blocks cause compilation failure
- **No magic numbers** - All numeric literals must be named constants
- Catches violations during:
  - `cargo build`
  - `cargo clippy`
  - CI/CD pipelines

## Benefits

### Memory Safety
Rust's ownership system guarantees:
- No use-after-free
- No double-free
- No dangling pointers
- No buffer overflows
- No data races

### Thread Safety
Safe Rust ensures:
- `Send`/`Sync` traits enforced by compiler
- No shared mutable state without synchronization
- Channels and mutexes are safe by default
- Deadlock-prone patterns caught at compile time

### Security
Safe Rust eliminates:
- **70% of CVEs** that affect C/C++ programs
- Entire CWE categories: CWE-119, CWE-416, CWE-476, etc.
- Memory corruption exploits
- Race condition vulnerabilities

### Maintainability
Benefits for development:
- No need to audit unsafe blocks
- Refactoring is safe by default
- New contributors can't introduce memory bugs
- Compiler is your safety net

## When Dependencies Use Unsafe

Some dependencies (like `portable-pty`) internally use `unsafe` to interact with OS APIs. This is acceptable when:

1. **Public API is safe**: We only call safe functions
2. **Well-maintained**: Dependency is actively maintained and audited
3. **Necessary**: No safe alternative exists for OS interaction
4. **Encapsulated**: Unsafe code is isolated and reviewed by experts

We **never** call:
- `unsafe` functions from dependencies
- FFI functions directly
- C libraries without safe wrappers

## How to Maintain This

### For New Code
1. Write all code in safe Rust
2. If you think you need `unsafe`, you probably don't
3. Use standard library safe abstractions instead
4. Ask for code review if uncertain

### For Dependencies
Before adding a new dependency:
1. Check if it exposes unsafe functions in public API
2. Verify it's well-maintained
3. Look for safer alternatives
4. Document why it's needed

### For Concurrency
Use safe patterns:
- `std::sync::mpsc` channels for message passing
- `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared state
- `std::thread::spawn` for thread creation
- Never use raw threading primitives

### For Performance
Safe Rust is fast! Optimizations without `unsafe`:
- Zero-cost abstractions
- Iterator chains with `map`/`filter`/`fold`
- Compiler optimizations (LLVM)
- `Vec`, `Box`, `Rc` are highly optimized

## Code Review Checklist

Reviewers MUST verify:
- [ ] No `unsafe` keyword anywhere
- [ ] No FFI calls
- [ ] No raw pointer types (`*const`, `*mut`)
- [ ] Dependencies don't expose unsafe public APIs
- [ ] Threading uses safe primitives
- [ ] Build passes with `#![forbid(unsafe_code)]`

## Examples

### ❌ FORBIDDEN
```rust
// NO: Raw pointer manipulation
let x = 5;
let ptr = &x as *const i32;
unsafe { *ptr }

// NO: Unchecked operations
unsafe { std::hint::unreachable_unchecked() }

// NO: Manual memory management
unsafe { std::alloc::alloc(...) }
```

### ✅ CORRECT
```rust
// YES: Use standard library safe types
let x = Box::new(5);
let value = *x;

// YES: Safe error handling
value.checked_add(1).expect("overflow");

// YES: Safe collections
let mut vec = Vec::new();
vec.push(42);
```

## Verification

To verify the policy is enforced:

```bash
# Should compile (no unsafe code)
cargo build

# Should pass (static analysis)
cargo clippy

# Grep for unsafe (should find nothing in src/)
grep -r "unsafe" src/
```

## Exceptions

**There are NO exceptions to this policy.**

If you believe you need unsafe code:
1. You're probably wrong - find a safe alternative
2. If truly necessary, move it to a separate crate with extensive documentation
3. That crate will NOT be part of this repository

## Questions?

- **"But library X uses unsafe internally"** - That's fine as long as its public API is safe
- **"Unsafe would be 10% faster"** - Safety > speed, and safe Rust is already fast
- **"I need to call C code"** - Use a safe wrapper crate (e.g., `nix` instead of `libc`)
- **"This would be simpler with unsafe"** - It would also be buggier

## References

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - When NOT to use unsafe
- [Rust Security Database](https://rustsec.org/) - CVEs that unsafe code enabled
- [Clippy Lint List](https://rust-lang.github.io/rust-clippy/master/) - Safety lints

---

**Remember: If it doesn't compile with `#![forbid(unsafe_code)]`, it doesn't ship.**
