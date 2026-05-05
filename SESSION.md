## Session 1 - day 1 - 2026/04/27

### What I built
- Proto-Value struct with data and label file
- A print helper function
- If passed by value: borrow, if passed by reference: no borrow

### What confused me
- Zed debugger

### What's next
- Day 2: add children: Vec<Value> to Value, try to build a graph 


## Session 2 - day 2 - 2026/04/28
### What I built/learnt
- Borrow: construct a graph that moves a variable then re-access it
- Make an enum to show which operation produces this variable
- Write a recursive function to print out the type of function

### What confused me
- clone() -> we need to implement it

### What's next
- Day 2 exercism.

## Session 3 - day 2 - 2026/04/29
### What I built/learnt
- Calculator RPN with stack. Mostly fill in the skeleton code

### What confused me
- (Rust module system is tied to crate structure, not file paths

### What's next
- Day 3 weekend — read Book 15.4–15.5, rebuild Value with Rc<RefCell<>>

## Session 4 - day 3 - 2026/05/03
### What I built/learnt
- Some small ownership exercise
  - can't reference something out of scope
  - one mutable reference at a time
  - can't refer to a mutable reference with immutable one

### What confused me
- Nothing

### What's next
- Day 3 weekend — read Book 15.4–15.5, rebuild Value with Rc<RefCell<>>


## Session 5 - day 3 - 2026/05/04
### What I built/learnt
- Change of plan: do [Matrix on Exercism](https://exercism.org/tracks/rust/exercises/matrix) instead.
- How to declare a `Struct` and implement its functions via `Impl`.
- Learn how to use `map` and `collect`.

### What confused me
- `into_iter` takes ownership, `iter` doesn't.

### What's next
- Day 4 - define a `Forward` trait and implement it for `AddNode`, `MulNode`, and `TanhNode`. Then operator overloading with `std::ops::Add`. Read Book chapters 10.1-10.2 first.
