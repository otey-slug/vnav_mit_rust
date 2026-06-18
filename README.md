# VNAV Rust

A Rust-first reimplementation of selected concepts from MIT 16.485 / VNAV: Visual Navigation for Autonomous Vehicles.

This repository is **not an official MIT course repository**. It is an independent learning project that rebuilds the core ideas of the VNAV labs in Rust, with an emphasis on safe robotics software, ROS 2 Rust nodes, geometric math, trajectory generation, visual odometry, and SLAM foundations.

## Goals

- Recreate the VNAV lab progression using Rust-first tooling.
- Build reusable Rust crates for robotics math, transforms, trajectories, and estimation.
- Use ROS 2 Rust where practical, especially for nodes, topics, and transform-style workflows.
- Keep each lab runnable as a small standalone project with tests and examples.
- Document design tradeoffs between the original C++/Python ecosystem and Rust alternatives.

## Original inspiration

MIT VNAV is publicly available and licensed under CC BY 4.0. This repo should cite and link to the original course pages rather than copying large portions verbatim.

Original course: <https://vnav.mit.edu/>
Original lab source code: <https://github.com/MIT-SPARK/VNAV-labs>

## Proposed roadmap

| Lab | Rust module | Status | Main focus |
|---|---|---:|---|
| Lab 1 | `docs/labs/lab1_rust_basics.md` | Draft | Rust basics, Cargo, `Vec`, structs, `nalgebra` |
| Lab 2 | `docs/labs/lab2_ros2_rust.md` | Draft | ROS 2 Rust, topics, transforms, homogeneous transformations |
| Lab 3 | `crates/vnav_control` | Planned | Quadrotor geometric control |
| Lab 4 | `crates/vnav_trajectory` | Planned | Polynomial/min-snap trajectory generation |
| Lab 5 | `crates/vnav_features` | Planned | Feature detection, matching, tracking |
| Lab 6 | `crates/vnav_vo` | Planned | Two-view motion estimation and RANSAC |
| Lab 7 | `crates/vnav_factor_graph` | Planned | Nonlinear least squares and factor graphs |
| Lab 8 | `crates/vnav_bow` | Planned | Bag-of-words place recognition |
| Lab 8.5 | `crates/vnav_object_slam` | Stretch | Object localization with detections and projection factors |
| Lab 9 | `docs/labs/lab9_slam_theory.md` | Planned | SLAM sparsity, marginalization, ORB-SLAM/Kimera concepts |
| Lab 9.5 | `docs/labs/lab9_5_slam_eval.md` | Stretch | Evaluation wrapper for ORB-SLAM/Kimera-style pipelines |

## Repository layout

```text
vnav-rust/
├── README.md
├── LICENSE
├── NOTICE.md
├── Cargo.toml
├── crates/
│   └── vnav_math/
│       ├── Cargo.toml
│       └── src/lib.rs
├── docs/
│   └── labs/
│       ├── lab1_rust_basics.md
│       └── lab2_ros2_rust.md
├── ros2_ws/
│   └── src/
│       └── vnav_lab2_rust/
│           ├── package.xml
│           ├── Cargo.toml
│           └── src/main.rs
└── .github/workflows/rust.yml
```

## Getting started

### Pure Rust crates

```bash
cargo test
```

### ROS 2 Rust workspace

The ROS 2 package under `ros2_ws/` assumes a working ROS 2 installation and a compatible Rust ROS 2 setup.

```bash
cd ros2_ws
source /opt/ros/jazzy/setup.bash   # or humble, depending on your system
colcon build --symlink-install
source install/setup.bash
ros2 run vnav_lab2_rust two_drone_tf
```

## Suggested naming

Good repo names:

- `vnav-rust`
- `rust-vnav-labs`
- `visual-navigation-rs`
- `vnav-rs`

Recommended: **`vnav-rust`** because it is clear, searchable, and not pretending to be the official course repo.
