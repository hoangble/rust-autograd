# Rust Autograd — Self-Study Plan

A phase-based, part-time learning plan for building a scalar autograd engine in Rust,
culminating in a generic tensor-capable system that can train a small MLP.

No fixed deadline. Each phase has an exit criterion — don't move on until it's met.

---

## Repo Structure

```
autograd-rs/
  NOTES.md              ← this file
  SESSION.md            ← updated every session (what you did, what confused you, what's next)
  Cargo.toml
  src/
    main.rs
    graph.rs            ← ValueRef, Value, backward()
    ops.rs              ← Add, Mul, tanh, relu, pow, etc.
    nn.rs               ← Neuron, Layer, MLP
  tests/
    finite_diff.rs      ← correctness tests for every op
  scratch/
    main.rs             ← throwaway experiments, never production code
  data/
    xor.csv             ← or hardcoded in code
```

---

## Scheduling Rules

**Weekday evenings (1–1.5 hrs):** Reading, Exercism exercises, small isolated experiments.
Not for half-finished implementations — too easy to lose context overnight.

**Weekends (3–4 hrs per day):** Project construction. Saturday builds, Sunday consolidates
(refactor, test, write SESSION.md).

**The one hard rule:** Never leave a half-finished implementation to a weekday evening.
Always reach a stable checkpoint before closing your laptop — it compiles, the failing test
has a clear next step in a comment, and SESSION.md is updated.

---

## SESSION.md Habit

At the end of every session, write 3–5 bullets:
- What you built
- What confused you
- What you're doing next

Read it before every new session. Five minutes of writing saves thirty minutes of
re-orientation. This habit is load-bearing on an open-ended schedule.

---

## References

| Resource | Role |
|---|---|
| [The Rust Book](https://doc.rust-lang.org/book/) | Primary reference, weeks 1–2 |
| [Rustlings](https://github.com/rust-lang/rustlings) | Short exercises for low-energy evenings |
| [Exercism Rust track](https://exercism.org/tracks/rust) | Targeted exercises per phase (see below) |
| [micrograd](https://github.com/karpathy/micrograd) | Read once before Phase 2. Then close it. |
| [Rust for Rustaceans](https://nostarch.com/rust-rustaceans) | Read ch.1 after Phase 3 |
| [Candle](https://github.com/huggingface/candle) | Skim for idioms after Phase 3 |
| Compiler errors | Every day. Read them completely, always. |

> **The Rust port of micrograd exists on GitHub. Don't look at it until you're done.**

---

## Guiding Principles

**The compiler is your pair programmer.** Don't fight error messages — read them completely.
Rust's errors are often more instructive than documentation.

**The math is your advantage.** You know backprop cold. Every time the language gets hard,
the domain is free. Use that asymmetry.

**Tests are your ground truth.** Finite difference checks on every op. If ∂f/∂x from your
graph matches `(f(x+ε) - f(x-ε)) / 2ε` to 5 decimal places, your autograd is correct
regardless of whether the code looks pretty.

**Write code first.** When it doesn't compile or doesn't do what you expect, *then* open
the book to the relevant section. The compiler error is the reading prompt.

---

## Phase 1 — Language Foundations

**Exit criterion:** You can write a struct with methods, pass references without compiler
errors, and explain ownership to yourself out loud.

### Exercism Targets
Do these in order, one or two per evening:
- `hello-world` — Cargo basics
- `rpn-calculator` — Enums, match, stack logic
- `matrix` — Vec<Vec<T>>, indexing, borrowing
- `clock` — Structs, impl, operator overloading
- `accumulate` — Closures, iterators
- `forth` — Complex enums, error handling
- `robot-simulator` — Structs, enums, methods combined

After submitting each exercise, read two community solutions.

### Day 1 — Proto-Value struct and first ownership error

Don't read anything first. Try to write this:

```rust
struct Value {
    data: f64,
    label: String,
}

fn print_value(v: Value) {
    println!("{}: {}", v.label, v.data);
}

fn main() {
    let v = Value { data: 3.0, label: String::from("x") };
    print_value(v);
    print_value(v); // try this
}
```

The second `print_value(v)` won't compile. Read the error. *Then* read Book chapters 1–3.
End the session with a `Value` struct you can create, print, and pass around without errors.

### Day 2 — Give Value children, discover the move problem

Add a children field:

```rust
struct Value {
    data: f64,
    label: String,
    children: Vec<Value>,
}
```

Try to build a graph:
```rust
let a = Value { data: 2.0, label: "a".into(), children: vec![] };
let b = Value { data: 3.0, label: "b".into(), children: vec![] };
let c = Value { data: 5.0, label: "c".into(), children: vec![a, b] };
```

Try to use `a` after putting it into `c.children`. Watch it fail. Read Book chapters 5–6.

Add an `Op` enum:
```rust
enum Op { Add, Mul, Tanh, Leaf }
```

Store it in `Value`. Write a `match` that prints what operation produced each node.

> You've now discovered the exact problem that `Rc<RefCell<>>` solves — organically.

### Day 3 — Fight the borrow checker on purpose

Read Book chapter 4 *first* today. Then do these exercises in order:

```rust
// Exercise 1: move vs borrow
fn add_data(v: Value) -> f64 { v.data }
let a = Value { data: 1.0, .. };
let _ = add_data(a);
println!("{}", a.data); // fails — fix with &Value

// Exercise 2: two mutable borrows
let mut a = Value { data: 1.0, .. };
let r1 = &mut a;
let r2 = &mut a; // fails — understand why

// Exercise 3: borrow outliving owner
let r: &Value;
{
    let a = Value { data: 1.0, .. };
    r = &a; // fails
}
println!("{}", r.data);
```

Fix each one. Write a comment explaining *why* each fix works. Don't move on until you can
predict whether code will compile before running it.

### Day 4 — Define a Forward trait and implement it

Read Book chapters 10.1–10.2. Then build:

```rust
trait Forward {
    fn forward(&self) -> f64;
}

struct AddNode { left: f64, right: f64 }
struct MulNode { left: f64, right: f64 }
struct TanhNode { input: f64 }

impl Forward for AddNode { fn forward(&self) -> f64 { self.left + self.right } }
impl Forward for MulNode { fn forward(&self) -> f64 { self.left * self.right } }
impl Forward for TanhNode { fn forward(&self) -> f64 { self.input.tanh() } }
```

Write `fn compute(node: &dyn Forward) -> f64`. Build a `Vec<Box<dyn Forward>>` with all
three types. Iterate over it calling `.forward()`.

Then overload `+`: implement `std::ops::Add` for `AddNode`. Notice the signature consumes
both operands. Try `impl Add for &AddNode` instead.

### Day 5 — Write a backward closure and box it

This is the most important day of Phase 1.

The backward for `c = a * b` should be:
- `a.grad += b.data * c.grad`
- `b.grad += a.data * c.grad`

Try to write this as a closure. You'll immediately hit the mutation problem with `move`
closures. Try different approaches and read what the compiler says about each.

Then solve it using `Rc<RefCell<f64>>`:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let a_grad = Rc::new(RefCell::new(0.0_f64));
let b_grad = Rc::new(RefCell::new(0.0_f64));
let a_data = 2.0_f64;
let b_data = 3.0_f64;
let c_grad = 1.0_f64;

let a_grad_clone = Rc::clone(&a_grad);
let b_grad_clone = Rc::clone(&b_grad);

let backward: Box<dyn Fn()> = Box::new(move || {
    *a_grad_clone.borrow_mut() += b_data * c_grad;
    *b_grad_clone.borrow_mut() += a_data * c_grad;
});

backward();
println!("a.grad = {}", a_grad.borrow()); // should be 3.0
println!("b.grad = {}", b_grad.borrow()); // should be 2.0
```

Get this working. Understand every line.

> This is literally the backward closure for multiplication. You've written a working
> piece of the autograd engine on Day 5.

### Day 6 — Error handling and dataset loading

Read Book chapter 9.

Write a function that reads a CSV of floats from disk and returns them as
`Vec<(f64, f64)>`. Use `anyhow::Result` as the return type. Handle: file not found,
malformed rows, non-numeric values.

```rust
fn load_dataset(path: &str) -> anyhow::Result<Vec<(f64, f64)>> {
    // your implementation
}
```

Then write a hardcoded XOR dataset returning the same type — this is what week 4 uses.

Read Book chapter 13 (closures and iterators). Practice chaining
`.iter().filter().map().collect()`.

### Day 7 — Wire it together: hand-traced computation

Don't read anything new.

Build `L = (a * b) + c` where `a=2.0, b=3.0, c=1.0`. Use `Rc<RefCell<f64>>` for all
grad values. Write each backward closure explicitly. Call them in reverse order (output
first, inputs last). Verify:

- `dL/da = b = 3.0`
- `dL/db = a = 2.0`
- `dL/dc = 1.0`

Then look at what you have and ask: what would it take to make this *automatic*? What
needs to be stored in a struct instead of written manually? Write down the answers as
comments.

> You've now run reverse-mode autodiff by hand in Rust. Week 3 is just automating this.

---

## Phase 2 — Autograd Primitives

**Exit criterion:** `let c = &a * &b; backward(&c);` correctly updates `a.grad` and
`b.grad`, confirmed by a passing `#[test]`.

### Exercism Targets
- `circular-buffer` — do this before *and* after building ValueRef. It's the most
  relevant exercise in the whole track.
- `simple-linked-list` — ownership, `Option<Box<T>>`
- `rectangles` — 2D iteration, lifetime intuition

### Day 8 — Design the Value struct properly, hit the graph problem

Try to build a proper graph with what you know:

```rust
struct Value {
    data: f64,
    grad: f64,
    children: Vec<Value>,
}

fn mul(a: Value, b: Value) -> Value {
    Value { data: a.data * b.data, grad: 0.0, children: vec![a, b] }
}
```

Try `let c = mul(a, b)` then `let d = mul(a, c)` — reusing `a` in two operations.
It fails: `a` was moved into `c`.

Read Book chapter 15.4 (`Rc<T>`). Rewrite `children` as `Vec<Rc<Value>>`. Verify the
same node can feed into two operations.

Define: `type ValueRef = Rc<RefCell<Value>>` — even if `RefCell` is still fuzzy.

### Day 9 — Add gradient mutation through shared references

Try to mutate `grad` through an `Rc<Value>`:

```rust
let a: Rc<Value> = Rc::new(Value { data: 2.0, grad: 0.0, children: vec![] });
a.grad += 1.0; // fails
```

Read Book chapter 15.5 (`RefCell<T>`). Wrap with `RefCell`:

```rust
let a: Rc<RefCell<Value>> = Rc::new(RefCell::new(Value { data: 2.0, grad: 0.0, children: vec![] }));
a.borrow_mut().grad += 1.0;
println!("{}", a.borrow().grad); // 1.0
```

Build a small three-node graph using `Rc<RefCell<Value>>` throughout. Write a backward
closure that mutates `grad` through the `Rc<RefCell<>>`. Confirm gradients update.

> By end of this session you should have a working `ValueRef = Rc<RefCell<Value>>` with
> actual gradient mutation through shared references.

### Day 10 — Implement Add and Mul on ValueRef

Read Book chapters 10 (generics) fully.

Implement operator overloading so `&a + &b` builds a new graph node with correct children
and a wired-up backward closure:

```rust
impl std::ops::Add for &ValueRef {
    type Output = ValueRef;
    fn add(self, rhs: &ValueRef) -> ValueRef {
        let out_data = self.borrow().data + rhs.borrow().data;

        let self_clone = Rc::clone(self);
        let rhs_clone = Rc::clone(rhs);
        let out = Rc::new(RefCell::new(Value {
            data: out_data,
            grad: 0.0,
            children: vec![Rc::clone(self), Rc::clone(rhs)],
            backward: Box::new(|| {}), // placeholder
        }));

        let out_clone = Rc::clone(&out);
        out.borrow_mut().backward = Box::new(move || {
            let grad = out_clone.borrow().grad;
            self_clone.borrow_mut().grad += grad;
            rhs_clone.borrow_mut().grad += grad;
        });

        out
    }
}
```

Test: `let c = &a + &b`, set `c.borrow_mut().grad = 1.0`, call
`(c.borrow().backward)()`, verify `a` and `b` gradients.

Implement `Mul` the same way.

### Day 11 — Topological sort

Write a function that takes a `ValueRef` and returns all nodes in topological order
(children before parents):

```rust
fn topological_sort(root: &ValueRef) -> Vec<ValueRef> {
    let mut visited = std::collections::HashSet::new();
    let mut order = Vec::new();
    // DFS — use Rc::as_ptr() cast to usize for HashSet keys
    order
}
```

Read Book chapter 8 (collections) while working with `HashSet` and `Vec`.

Test on a hand-built graph. Verify the output order by printing labels.

### Day 12 — A working .backward() method

Add a `backward()` function that:
1. Sets `root.grad = 1.0`
2. Gets the topological order
3. Iterates in reverse, calling each node's backward closure

Test on `c = a * b`: call `backward(&c)`, assert:
- `a.borrow().grad ≈ b.borrow().data`
- `b.borrow().grad ≈ a.borrow().data`

Then test `d = (a * b) + c` with concrete values. Compute expected gradients by hand
first, then run `.backward()` and compare.

### Day 13 — Split into modules, add tanh

Organize into:

```
src/
  main.rs
  graph.rs    ← ValueRef, Value, backward()
  ops.rs      ← Add, Mul, tanh, relu
  nn.rs       ← empty for now
```

Read Book chapter 7 (modules). Practice `pub`, `pub(crate)`, `use`, `mod`.

Add `tanh` as a method. Run `cargo clippy`. Fix every warning.

### Day 14 — Finite difference tests for existing ops

Write proper `#[test]` functions:

```rust
#[test]
fn test_mul_backward() {
    let a = Value::new(2.0);
    let b = Value::new(3.0);
    let c = &a * &b;
    backward(&c);

    let eps = 1e-5;
    let numerical = ((2.0 + eps) * 3.0 - (2.0 - eps) * 3.0) / (2.0 * eps);
    assert!((a.borrow().grad - numerical).abs() < 1e-4);
}
```

Run `cargo test`. Fix anything that fails. Both `add` and `mul` backward passes should
be confirmed correct by automated tests before moving to Phase 3.

---

## Phase 3 — Complete Autograd Engine

**Exit criterion:** `cargo test` passes finite difference checks for every op — `add`,
`mul`, `tanh`, `relu`, `pow`, subtraction, division, negation.

No new Exercism this phase. After your engine passes all tests, skim
[Candle](https://github.com/huggingface/candle) for idioms.

### Remaining ops

| Op | Backward formula |
|---|---|
| `add(a, b)` | `a.grad += out.grad; b.grad += out.grad` |
| `mul(a, b)` | `a.grad += b.data * out.grad; b.grad += a.data * out.grad` |
| `tanh(a)` | `a.grad += (1 - out.data²) * out.grad` |
| `relu(a)` | `a.grad += (out.data > 0) as f64 * out.grad` |
| `pow(a, n)` | `a.grad += n * a.data^(n-1) * out.grad` |
| `neg(a)` | `a * -1` |
| `sub(a, b)` | `a + neg(b)` |
| `div(a, b)` | `a * pow(b, -1)` |

### Stress test

Build a chain of 20+ ops and call `.backward()`. Check gradients still look right.
Verify topological sort handles the full graph. Confirm `cargo test` stays green.

---

## Phase 4 — MLP and Training

**Exit criterion:** A binary that trains on XOR, prints decreasing loss, and terminates
with >99% accuracy.

### Neuron, Layer, MLP

```rust
struct Neuron {
    weights: Vec<ValueRef>,
    bias: ValueRef,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct MLP {
    layers: Vec<Layer>,
}
```

Initialize weights with small random values using the `rand` crate. Implement `forward`
on each. A `Neuron` computes `tanh(w · x + b)`.

### Loss and SGD

```rust
fn mse(predictions: &[ValueRef], targets: &[f64]) -> ValueRef {
    // sum of (pred - target)^2 / n
}
```

SGD: after `.backward()`, update each parameter:
```rust
param.borrow_mut().data -= learning_rate * param.borrow().grad;
param.borrow_mut().grad = 0.0;
```

Write a helper that collects all parameters from an MLP as `Vec<ValueRef>`.

### XOR

```
inputs:  [0,0], [0,1], [1,0], [1,1]
targets: [0],   [1],   [1],   [0]
```

Train a 2→4→1 MLP with tanh activations. Should converge in 200–500 steps. Print loss
every 10 steps.

When loss drops to near zero, your entire system — autograd, backward pass, SGD — is
working correctly.

### Benchmark

Write equivalent Python using PyTorch (`requires_grad=True`, manual backward) performing
the same MLP forward + backward pass. Time both with `std::time::Instant` in Rust and
`time.perf_counter()` in Python. Print microseconds per step.

---

## Phase 5 — Tensor Lift

**Exit criterion:** `Value<T>` is generic, `T = ndarray::Array1<f64>` works, batched
forward and backward pass runs correctly.

This is a full phase, not a stretch goal. Plan for 3–4 weekends. The compiler errors
will be longer and less friendly than anything before this.

### Generic Value

Replace `f64` in `Value` with a generic `T`:

```rust
struct Value<T> {
    data: T,
    grad: T,
    backward: Box<dyn Fn()>,
    children: Vec<Rc<RefCell<Value<T>>>>,
}
```

Trait bounds: `T: Add<Output=T> + Mul<Output=T> + Clone + Zero`

Read Book chapter 10 fully again — after Phase 3, it will read differently.

### What changes

- Element-wise ops generalize directly from scalar
- Batched forward passes work without looping over scalar values
- `tanh` and `relu` need element-wise `ndarray` equivalents
- The `mse` loss becomes a sum over array elements

### What doesn't change

The graph structure — `Rc<RefCell<Value<T>>>`, topological sort, backward closure
wiring — is identical. You're parameterizing the arithmetic, not the graph.

---

## Phase 6 — Hardening and Extension

**Exit criterion:** README written, real dataset beyond XOR trains correctly, no
`unwrap()` calls, `cargo test` still green.

### Hardening checklist

- [ ] Replace all `unwrap()` / `expect()` with proper `anyhow::Result` propagation
- [ ] `cargo clippy` — zero warnings
- [ ] `cargo test` — all finite difference tests pass
- [ ] `cargo build --release` — binary is small and fast
- [ ] `SESSION.md` reflects the full project history

### Real dataset

Generate 200 points from two interlocking spirals (standard 2D binary classification
benchmark), or load the sklearn moons dataset as a CSV. Train your MLP. Watch accuracy
climb.

### Parallelism experiment (optional)

Add `rayon`. Try `.par_iter()` on the topological sort traversal. Be careful: gradient
accumulation requires mutable access, which `rayon` will reject unless you partition the
work correctly. This may not pan out cleanly — parallel autograd is a research problem.
The attempt teaches you more about `Send`, `Sync`, and Rust's concurrency model than
any tutorial.

### README

Write a README that explains:
- What autograd is and why it matters
- The key design decision: why `Rc<RefCell<Value>>`
- What you'd do differently with more time (arena allocator? GPU via `wgpu`?)
- How to run the XOR training example

Then read *Rust for Rustaceans* chapter 1. After six phases of fighting ownership in a
real project, it will feel like someone finally explaining the rules of a game you've
already been playing.

---

## Exercism Quick Reference

| Exercise | Phase | Concept |
|---|---|---|
| `hello-world` | 1 | Cargo basics |
| `rpn-calculator` | 1 | Enums, match, stack |
| `matrix` | 1 | Vec<Vec<T>>, borrowing |
| `clock` | 1 | Structs, operator overloading |
| `accumulate` | 1 | Closures, iterators |
| `forth` | 1 | Complex enums, error handling |
| `robot-simulator` | 1 | Structs + enums combined |
| `circular-buffer` | 2 | RefCell, interior mutability |
| `simple-linked-list` | 2 | Ownership, Option<Box<T>> |
| `rectangles` | 2 | Lifetime intuition |

> Do `circular-buffer` before *and* after building `ValueRef`. It's a different
> exercise once you've seen why `RefCell` exists in a real project.

---

## Common Failure Modes

**"I read a lot but didn't build anything."**
Exercism in the evenings is not a substitute for project work on weekends. Reading the
Book is not building. If a weekend passes without a stable checkpoint, the week was lost.

**Losing context between sessions.**
Read SESSION.md before every session. Write it after every session. Non-negotiable.

**Staying stuck too long alone.**
If you're stuck on the same thing for two full weekend sessions, look at one community
solution on Exercism, or read the relevant chapter of Rust for Rustaceans. Struggling
productively is good; spinning is not.

**Skipping the finite difference tests.**
Don't proceed past Phase 2 until `cargo test` is green. An autograd bug discovered in
Phase 4 during training takes ten times longer to debug than one caught in Phase 2 by a
focused unit test.

**Opening the micrograd Rust port.**
Don't. There's nothing wrong with it — it will just short-circuit the learning.
The moment you copy a pattern you didn't discover yourself, you stop building intuition
and start building a copy.