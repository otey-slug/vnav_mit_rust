# Rust

## Table of contents

- [How Rust works](#how-rust-works)
- [Hello world](#hello-world)
- [`Vec<T>`](#vect)
- [Structs and `impl` blocks](#structs-and-impl-blocks)
- [`nalgebra`](#nalgebra)
- [Where to go now](#where-to-go-now)

---

## How Rust works

> **Warning**  
> This section should hopefully serve as a refresher. If you are completely new to programming, the terminal, or compiled languages, ask for extra resources before going too far.

Rust is a high-level systems programming language. A computer's CPU cannot directly understand Rust source code. The limited set of instructions that a CPU can execute directly is called **machine code**.

Programs written in high-level languages must be translated before they can run. There are two common ways this happens:

1. **Compilation** — source code is translated into an executable ahead of time.
2. **Interpretation** — source code is read and executed by another program at runtime.

Rust is a **compiled language**, meaning your `.rs` files are compiled into machine code before you run them.

Rust is often compared with C and C++, but it makes different tradeoffs:

- Rust has **no garbage collector**.
- Rust gives you low-level control over memory.
- Rust checks memory safety through **ownership**, **borrowing**, and **lifetimes**.
- Rust projects are usually managed with **Cargo**, Rust's build tool and package manager.

For this lab, you do not need to master all of Rust. The goal is to get comfortable with enough Rust syntax to write small robotics and VNAV-style programs.

### Recommended setup

Check that Rust and Cargo are installed:

```bash
rustc --version
cargo --version
```

If these commands fail, install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal or run:

```bash
source "$HOME/.cargo/env"
```

---

## Hello world

It is not a programming language if it does not let you print `"Hello World"`.

The first Rust program we will look at prints a simple message to the terminal.

Create a new Rust project:

```bash
cargo new hello_rust
cd hello_rust
```

Cargo will create a directory that looks like this:

```text
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

Open `src/main.rs` and replace its contents with:

```rust
fn main() {
    println!("Hello World from VNAV in Rust!");
}
```

Run the program:

```bash
cargo run
```

You should see something like:

```text
Hello World from VNAV in Rust!
```

### What is happening?

The program starts with:

```rust
fn main() {
```

Every executable Rust program has a `main` function. This function is the entry point of the program. When you run the executable, the operating system starts your program by calling `main`.

The line:

```rust
println!("Hello World from VNAV in Rust!");
```

prints text to the terminal.

`println!` ends with an exclamation mark because it is a **macro**, not a normal function. Macros are a Rust feature that generate code before compilation. For now, you can think of `println!` as the Rust equivalent of printing to standard output.

The braces `{` and `}` define the body of the function.

Unlike C++, Rust does not require `return 0;` at the end of `main`. A normal `main` function that finishes successfully exits with success automatically.

### Compiling directly with `rustc`

Cargo is the normal way to build Rust projects, but you can also compile a single file manually.

Create a file called `hello.rs`:

```rust
fn main() {
    println!("Hello World from a single Rust file!");
}
```

Compile it:

```bash
rustc hello.rs
```

Run it:

```bash
./hello
```

For most real projects, prefer Cargo.

> **Keep in mind**  
> In Rust, use `cargo run`, `cargo build`, `cargo test`, and `cargo check` frequently. `cargo check` is especially useful because it type-checks your code without fully building an executable.

---

## `Vec<T>`

Let us move to something more useful: the standard vector.

In Rust, `Vec<T>` is a growable array type provided by the standard library. It stores values of type `T` contiguously in memory and can grow as you push new elements into it.

This is similar in spirit to `std::vector<T>` in C++.

### Creating an empty vector

```rust
let mut vec: Vec<i32> = Vec::new();
```

There are a few important Rust ideas in this line:

- `let` creates a variable.
- `mut` means the variable is mutable.
- `Vec<i32>` means this vector stores 32-bit signed integers.
- `Vec::new()` constructs an empty vector.

Without `mut`, you would not be allowed to push new elements into the vector.

You can also write:

```rust
let mut vec = Vec::<i32>::new();
```

or, if Rust can infer the type later:

```rust
let mut vec = Vec::new();
```

### Creating a vector with initial values

The simplest way to create a vector with values is with the `vec!` macro:

```rust
let numbers = vec![1, 2, 3, 4, 5];
```

To create a vector of repeated values:

```rust
let histo = vec![0.0; 10];
```

This creates a vector with 10 elements, each initialized to `0.0`.

### Pushing values

If we do not pre-initialize the vector, we can add elements one by one using `push`.

Create a project:

```bash
cargo new vector_example
cd vector_example
```

Edit `src/main.rs`:

```rust
fn main() {
    let mut vec: Vec<i32> = Vec::new();

    for i in 1..=5 {
        vec.push(i * 10);
    }

    println!("Vector size: {}", vec.len());

    print!("Vector elements: ");
    for elem in &vec {
        print!("{} ", elem);
    }
    println!();
}
```

Run it:

```bash
cargo run
```

You should see:

```text
Vector size: 5
Vector elements: 10 20 30 40 50
```

### Indexing

You can access vector elements with square brackets:

```rust
let first = vec[0];
```

However, square bracket indexing will panic if the index is out of bounds.

A safer option is `.get()`:

```rust
match vec.get(0) {
    Some(value) => println!("First value: {}", value),
    None => println!("The vector is empty."),
}
```

`get` returns an `Option<&T>`.

This means:

- `Some(value)` if the element exists.
- `None` if the index is out of bounds.

> **Keep in mind**  
> Rust often makes possible failure explicit in the type system. Instead of silently returning invalid memory or crashing unpredictably, Rust uses types like `Option<T>` and `Result<T, E>`.

### Iteration and borrowing

This loop:

```rust
for elem in &vec {
    println!("{}", elem);
}
```

borrows the vector immutably. The `&vec` means "iterate over references to the elements without taking ownership of the vector."

If you wrote:

```rust
for elem in vec {
    println!("{}", elem);
}
```

then the loop would consume the vector. After that loop, you could no longer use `vec`.

This is one of the most important Rust differences from C++: ownership is checked by the compiler.

### Complete vector example

Replace `src/main.rs` with:

```rust
fn main() {
    let mut vec: Vec<i32> = Vec::new();

    for i in 1..=5 {
        vec.push(i * 10);
    }

    println!("Vector size: {}", vec.len());

    print!("Vector elements: ");
    for elem in &vec {
        print!("{} ", elem);
    }
    println!();

    match vec.get(2) {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element exists."),
    }
}
```

Run:

```bash
cargo run
```

Expected output:

```text
Vector size: 5
Vector elements: 10 20 30 40 50
Third element: 30
```

---

## Structs and `impl` blocks

Rust is not an object-oriented language in exactly the same way as C++, but it supports many similar ideas.

In Rust, you usually define data with a `struct` and define behavior using an `impl` block.

For example, imagine a new type called `Circle`.

A circle has a radius, and there are several useful operations we may want to perform:

- compute its circumference
- compute its area

### Declaring a struct

```rust
struct Circle {
    radius: f64,
}
```

This creates a new type named `Circle` with one field named `radius`.

We can create a `Circle` like this:

```rust
let circ = Circle { radius: 3.0 };
```

### Adding methods

To add methods, use an `impl` block:

```rust
impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}
```

Important details:

- `fn new(radius: f64) -> Self` is a constructor-style associated function.
- `Self` means the type currently being implemented, here `Circle`.
- `&self` means the method borrows the circle without taking ownership.
- `f64` is a 64-bit floating-point number.

### Complete circle example

Create a project:

```bash
cargo new circle_example
cd circle_example
```

Edit `src/main.rs`:

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

fn main() {
    let circ = Circle::new(3.0);

    println!("Circumference: {}", circ.circumference());
    println!("Area: {}", circ.area());
}
```

Run:

```bash
cargo run
```

You should see something close to:

```text
Circumference: 18.84955592153876
Area: 28.274333882308138
```

### Visibility

By default, struct fields and functions are private to their module.

For a small `main.rs`, this may not matter much. In a larger project, you may choose to make items public using `pub`.

Example:

```rust
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}
```

> **Keep in mind**  
> Prefer keeping fields private and exposing methods when you want control over how a type is constructed or modified. This is useful in robotics code where invalid states can create subtle bugs.

For example, you may not want a circle with a negative radius:

```rust
impl Circle {
    pub fn new(radius: f64) -> Result<Self, String> {
        if radius < 0.0 {
            Err("radius must be non-negative".to_string())
        } else {
            Ok(Self { radius })
        }
    }
}
```

---

## `nalgebra`

Now that we have covered some Rust basics, it is time to look at a library that will be useful for VNAV-style robotics work.

In C++, VNAV often uses Eigen for linear algebra. In Rust, a common Eigen-like choice is [`nalgebra`](https://nalgebra.org/).

`nalgebra` provides vectors, matrices, rotations, quaternions, transforms, and other tools that are useful in robotics, controls, estimation, and computer vision.

### Create a project

```bash
cargo new nalgebra_example
cd nalgebra_example
```

Add `nalgebra` to `Cargo.toml`:

```toml
[dependencies]
nalgebra = "0.33"
```

Your `Cargo.toml` should look something like:

```toml
[package]
name = "nalgebra_example"
version = "0.1.0"
edition = "2021"

[dependencies]
nalgebra = "0.33"
```

### Matrix example

Edit `src/main.rs`:

```rust
use nalgebra::DMatrix;

fn main() {
    let mut m = DMatrix::<f64>::zeros(2, 2);

    m[(0, 0)] = 3.0;
    m[(1, 0)] = 2.5;
    m[(0, 1)] = -1.0;
    m[(1, 1)] = m[(1, 0)] + m[(0, 1)];

    println!("{}", m);
}
```

Run:

```bash
cargo run
```

You should see a 2-by-2 matrix:

```text
  ┌         ┐
  │   3  -1 │
  │ 2.5 1.5 │
  └         ┘
```

The exact formatting may differ slightly depending on your `nalgebra` version.

### Fixed-size vectors

For robotics code, fixed-size vectors are often useful:

```rust
use nalgebra::Vector3;

fn main() {
    let position = Vector3::new(1.0, 2.0, 3.0);
    let velocity = Vector3::new(0.5, 0.0, -0.2);

    let next_position = position + velocity * 0.1;

    println!("position: {}", position);
    println!("next position: {}", next_position);
}
```

This is useful for representing positions, velocities, accelerations, forces, and other 3D quantities.

### Rotations

`nalgebra` also supports rotations:

```rust
use nalgebra::{Rotation3, Vector3};

fn main() {
    let point = Vector3::new(1.0, 0.0, 0.0);

    let yaw = std::f64::consts::FRAC_PI_2;
    let rotation = Rotation3::from_euler_angles(0.0, 0.0, yaw);

    let rotated_point = rotation * point;

    println!("rotated point: {}", rotated_point);
}
```

A yaw rotation of 90 degrees should rotate the x-axis toward the y-axis.

### Small VNAV-style exercise

Create a function that transforms a point from a body frame to a world frame:

```rust
use nalgebra::{Rotation3, Vector3};

fn transform_point(
    rotation_world_body: &Rotation3<f64>,
    translation_world_body: &Vector3<f64>,
    point_body: &Vector3<f64>,
) -> Vector3<f64> {
    rotation_world_body * point_body + translation_world_body
}

fn main() {
    let rotation_world_body = Rotation3::from_euler_angles(
        0.0,
        0.0,
        std::f64::consts::FRAC_PI_2,
    );

    let translation_world_body = Vector3::new(1.0, 2.0, 0.0);
    let point_body = Vector3::new(1.0, 0.0, 0.0);

    let point_world = transform_point(
        &rotation_world_body,
        &translation_world_body,
        &point_body,
    );

    println!("point in world frame: {}", point_world);
}
```

This is the type of math that appears constantly in visual navigation, state estimation, and robot control.

---

## Exercises

### Exercise 1: Hello Rust

Create a new Cargo project called `vnav_rust_intro`.

Modify `src/main.rs` so that it prints:

```text
Hello VNAV from Rust!
```

Run it with:

```bash
cargo run
```

Then run:

```bash
cargo check
cargo build
```

Answer the following questions in a short `README.md`:

1. What is the difference between `cargo check` and `cargo build`?
2. Where does Cargo place the compiled binary?
3. What file stores the package metadata and dependencies?

---

### Exercise 2: Vector statistics

Write a Rust program that creates a vector of floating-point values:

```rust
let data = vec![1.0, 2.0, 4.0, 8.0, 16.0];
```

Implement functions:

```rust
fn mean(data: &[f64]) -> f64
fn min_value(data: &[f64]) -> Option<f64>
fn max_value(data: &[f64]) -> Option<f64>
```

Use slices (`&[f64]`) instead of taking ownership of the vector.

Your program should print:

```text
mean: 6.2
min: 1
max: 16
```

---

### Exercise 3: Circle struct

Create a `Circle` struct with a private `radius` field.

Implement:

```rust
impl Circle {
    pub fn new(radius: f64) -> Result<Self, String>
    pub fn radius(&self) -> f64
    pub fn circumference(&self) -> f64
    pub fn area(&self) -> f64
}
```

The constructor should reject negative radii.

Test your code with:

```rust
let good_circle = Circle::new(3.0);
let bad_circle = Circle::new(-1.0);
```

---

### Exercise 4: Basic linear algebra

Create a project called `vnav_linalg`.

Add `nalgebra` to `Cargo.toml`.

Write a program that:

1. Creates two `Vector3<f64>` values.
2. Computes their sum.
3. Computes their dot product.
4. Computes their cross product.
5. Prints all results.

Example vectors:

```rust
let a = Vector3::new(1.0, 0.0, 0.0);
let b = Vector3::new(0.0, 1.0, 0.0);
```

Expected cross product:

```text
a x b = [0, 0, 1]
```

The exact formatting may differ.

---

### Exercise 5: Coordinate transform

Using `nalgebra`, write a function:

```rust
fn transform_point(
    rotation_world_body: &Rotation3<f64>,
    translation_world_body: &Vector3<f64>,
    point_body: &Vector3<f64>,
) -> Vector3<f64>
```

Then test it using:

- yaw angle: 90 degrees
- translation: `(1, 2, 0)`
- body-frame point: `(1, 0, 0)`

Check that the result is approximately:

```text
(1, 3, 0)
```

This is a simple version of transforming points between robot frames.

---

## Where to go now

Rust is a large language, but you do not need to learn all of it before writing useful robotics code.

Good next steps:

- Read chapters 1–6 of **The Rust Programming Language**.
- Learn `cargo test` and write unit tests for your math functions.
- Practice using `nalgebra` for `Vector3`, `Matrix3`, `Rotation3`, `UnitQuaternion`, and `Isometry3`.
- Try implementing a tiny VNAV transform library:
  - compose transforms
  - invert transforms
  - transform points
  - convert Euler angles to rotation matrices
  - test frame convention mistakes

Useful resources:

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [nalgebra documentation](https://nalgebra.org/)
- [Rust standard library documentation](https://doc.rust-lang.org/std/)
