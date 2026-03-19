# AnimaLab 🦀

**Personal Rust learning lab** — part of the [AnimaCore](https://github.com/M74X) ecosystem.

30-day curriculum, 1 topic per day. Every session feeds two things simultaneously:
1. **Personal mastery** — solidifying Rust for PAEChaka and AnimaCore development
2. **CodeShelter content** — these exercises become the foundation of future courses for underprivileged youth

---

## Structure

```
AnimaLab/
├── block0/   Fundamentals        (Days 01–07)
├── block1/   Data Structures     (Days 08–12)
├── block2/   PAEChaka Core       (Days 13–19)
├── block3/   Solid Rust          (Days 20–25)
└── block4/   AnimaCore Territory (Days 26–30)
```

---

## Curriculum

### Block 0 — Fundamentals
| Day | Topic |
|-----|-------|
| 01 | Variables, mutability, shadowing |
| 02 | Primitive types + type inference |
| 03 | Functions + expressions vs statements |
| 04 | Control flow |
| 05 | Ownership |
| 06 | Borrowing + references |
| 07 | Slices |

### Block 1 — Data Structures
| Day | Topic |
|-----|-------|
| 08 | Structs + impl |
| 09 | Enums + pattern matching |
| 10 | Option\<T\> |
| 11 | Result\<T, E\> + the ? operator |
| 12 | Vec, HashMap, String |

### Block 2 — PAEChaka Core
| Day | Topic |
|-----|-------|
| 13 | Traits + generics |
| 14 | Structs avanzados + Serde |
| 15 | Error handling sin unwrap() |
| 16 | async/await + Tokio |
| 17 | HTTP handlers (Axum) |
| 18 | UUID + Chrono |
| 19 | SQLx query patterns |

### Block 3 — Solid Rust
| Day | Topic |
|-----|-------|
| 20 | Lifetimes |
| 21 | Closures + advanced iterators |
| 22 | Smart pointers |
| 23 | Concurrency |
| 24 | Macros + testing |
| 25 | Workspaces + large modules |

### Block 4 — AnimaCore Territory
| Day | Topic |
|-----|-------|
| 26 | FFI: Rust ↔ C++ |
| 27 | Unsafe Rust |
| 28 | Procedural macros |
| 29 | Vulkan with Rust |
| 30 | WebAssembly (WASM) |

---

## How sessions work

Each day follows this protocol:
1. **Recap quiz** — questions about the previous topic
2. **Concept + analogy** — connected to real-world context
3. **Prediction** — read code, predict output before running
4. **Hands-on** — run it, explain why it works
5. **Intentional bug** — find and fix a deliberate error
6. **"Why does this work?"** — articulate the reasoning
7. **Mini challenge** — small real-world exercise to close

Sessions run in **Google Colab**, connected to this repo via Google Drive.

---

## Connection to AnimaCore

```
AnimaLab
    └── feeds ──> AnimaCore/core
                      ├── inductive-engine (C++)
                      ├── agent-system (Rust)
                      └── graphic-engine (Rust/Vulkan)
```

Block 4 topics (FFI, unsafe, Vulkan, WASM) are the direct bridge between AnimaLab and AnimaCore production code.

---

*Built by [@M74X](https://github.com/M74X) — Lima, Perú 🇵🇪*
