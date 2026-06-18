# Lab 2 — ROS 2 Rust, `tf`, and Homogeneous Transformations

> Adapted from MIT VNAV Lab 2 into a Rust-first ROS 2 exercise.
>
> Original Lab 2 topics: installing ROS 2, ROS nodes/topics, launch files, `tf2`, RViz, a two-drone rigid-transform scenario, and mathematical derivations for relative motion.
>
> This version keeps the same learning goals, but uses Rust ROS 2 nodes instead of C++ nodes.

---

## Table of contents

1. [Learning goals](#learning-goals)
2. [Prerequisites](#prerequisites)
3. [Installing ROS 2 and Rust ROS support](#installing-ros-2-and-rust-ros-support)
4. [ROS 2 concepts refresher](#ros-2-concepts-refresher)
5. [Creating a Rust ROS 2 workspace](#creating-a-rust-ros-2-workspace)
6. [Minimal Rust publisher and subscriber](#minimal-rust-publisher-and-subscriber)
7. [Launch files](#launch-files)
8. [Transforms and `tf`](#transforms-and-tf)
9. [The two-drone scenario](#the-two-drone-scenario)
10. [Deliverables](#deliverables)
11. [Math section](#math-section)
12. [Suggested repository layout](#suggested-repository-layout)
13. [Debugging checklist](#debugging-checklist)
14. [Where to go next](#where-to-go-next)

---

## Learning goals

By the end of this lab, you should be able to:

- Create and build a ROS 2 Rust package.
- Write Rust ROS 2 publisher and subscriber nodes.
- Use ROS 2 command-line tools to inspect nodes, topics, messages, and launch files.
- Publish coordinate-frame transforms for two simulated aerial vehicles.
- Visualize frames and robot meshes in RViz.
- Look up relative transforms between frames.
- Derive the relative trajectory of one drone as seen from another drone's body frame.

The original lab uses ROS 2 Humble and C++ examples. This Rust version assumes the same ROS 2 concepts, but the implementation language is Rust using `rclrs`.

---

## Prerequisites

You should already be comfortable with:

- Basic Linux terminal usage.
- Git.
- Rust basics: `cargo`, structs, ownership, `Result`, closures, `Arc`, and `Mutex`.
- Matrix/vector notation.
- Rotation matrices, quaternions, and homogeneous transformations.

Recommended Rust crates for this lab:

```toml
[dependencies]
rclrs = "0.7"
std_msgs = "*"
geometry_msgs = "*"
visualization_msgs = "*"
builtin_interfaces = "*"
nalgebra = "0.33"
```

> Keep in mind: `rclrs` is evolving quickly. APIs may differ slightly depending on your ROS 2 distro and installed crate versions. Treat the code snippets as a clean starting point, then adjust against your local `rclrs` docs if needed.

---

## Installing ROS 2 and Rust ROS support

### 1. Install ROS 2

For this lab, use ROS 2 Humble on Ubuntu 22.04, or ROS 2 Jazzy on Ubuntu 24.04 if that is your current development environment.

For Humble:

```bash
sudo apt update
sudo apt install ros-humble-desktop python3-colcon-common-extensions python3-rosdep python3-vcstool build-essential
source /opt/ros/humble/setup.bash
```

For Jazzy:

```bash
sudo apt update
sudo apt install ros-jazzy-desktop python3-colcon-common-extensions python3-rosdep python3-vcstool build-essential
source /opt/ros/jazzy/setup.bash
```

Add the setup file to your shell configuration:

```bash
# Humble
printf '\nsource /opt/ros/humble/setup.bash\n' >> ~/.bashrc

# or Jazzy
printf '\nsource /opt/ros/jazzy/setup.bash\n' >> ~/.bashrc
```

To keep ROS traffic local to your machine:

```bash
echo 'export ROS_LOCALHOST_ONLY=1' >> ~/.bashrc
source ~/.bashrc
```

Validate the ROS install:

```bash
ros2 run demo_nodes_cpp talker
```

In a second terminal:

```bash
ros2 run demo_nodes_py listener
```

You should see the listener receiving messages from the talker.

---

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version
cargo --version
```

---

### 3. Install Rust support for `colcon`

```bash
sudo apt install git libclang-dev python3-pip python3-vcstool
pip install --user colcon-cargo colcon-ros-cargo
```

Make sure your Python user scripts are on your path:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

### 4. Add Rust ROS support packages

Create a workspace that includes the Rust ROS support code and the message crates needed by this lab.

For ROS 2 Humble:

```bash
mkdir -p ~/vnav_rust_ws/src
cd ~/vnav_rust_ws

source /opt/ros/humble/setup.bash

git clone -b humble https://github.com/ros2/common_interfaces.git src/common_interfaces
git clone -b humble https://github.com/ros2/example_interfaces.git src/example_interfaces
git clone -b humble https://github.com/ros2/rcl_interfaces.git src/rcl_interfaces
git clone -b humble https://github.com/ros2/rosidl_core.git src/rosidl_core
git clone -b humble https://github.com/ros2/rosidl_defaults.git src/rosidl_defaults
git clone -b humble https://github.com/ros2/unique_identifier_msgs.git src/unique_identifier_msgs
git clone https://github.com/ros2-rust/rosidl_rust.git src/rosidl_rust
```

For ROS 2 Jazzy, use the `jazzy` branch for the ROS repositories:

```bash
mkdir -p ~/vnav_rust_ws/src
cd ~/vnav_rust_ws

source /opt/ros/jazzy/setup.bash

git clone -b jazzy https://github.com/ros2/common_interfaces.git src/common_interfaces
git clone -b jazzy https://github.com/ros2/example_interfaces.git src/example_interfaces
git clone -b jazzy https://github.com/ros2/rcl_interfaces.git src/rcl_interfaces
git clone -b jazzy https://github.com/ros2/rosidl_core.git src/rosidl_core
git clone -b jazzy https://github.com/ros2/rosidl_defaults.git src/rosidl_defaults
git clone -b jazzy https://github.com/ros2/unique_identifier_msgs.git src/unique_identifier_msgs
git clone https://github.com/ros2-rust/rosidl_rust.git src/rosidl_rust
```

Build the support workspace:

```bash
cd ~/vnav_rust_ws
colcon build --symlink-install
source install/setup.bash
```

---

## ROS 2 concepts refresher

### ROS nodes

A ROS node is a running process that performs one job. Examples:

- a camera driver node,
- a controller node,
- a transform broadcaster node,
- a visualizer node,
- a logger node.

List active nodes:

```bash
ros2 node list
```

Inspect a node:

```bash
ros2 node info /node_name
```

---

### ROS topics

A topic is a named communication channel. Publishers write messages to topics. Subscribers receive messages from topics.

List topics:

```bash
ros2 topic list
```

Show a topic's message type:

```bash
ros2 topic info /topic_name
```

Print messages:

```bash
ros2 topic echo /topic_name
```

Show message definition:

```bash
ros2 interface show geometry_msgs/msg/TransformStamped
```

---

### Turtlesim warmup

Terminal 1:

```bash
ros2 run turtlesim turtlesim_node
```

Terminal 2:

```bash
ros2 run turtlesim turtle_teleop_key
```

Terminal 3:

```bash
ros2 node list
ros2 topic list
ros2 topic echo /turtle1/cmd_vel
```

This is the quickest way to verify that your ROS graph, topics, and command-line tools are working.

---

## Creating a Rust ROS 2 workspace

Inside your existing workspace:

```bash
cd ~/vnav_rust_ws/src
cargo new two_drones_rust
cd two_drones_rust
mkdir launch config mesh
```

Your package will need both a `Cargo.toml` and a ROS `package.xml`.

### `Cargo.toml`

```toml
[package]
name = "two_drones_rust"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "frames_publisher_node"
path = "src/frames_publisher_node.rs"

[[bin]]
name = "plots_publisher_node"
path = "src/plots_publisher_node.rs"

[[bin]]
name = "minimal_publisher"
path = "src/minimal_publisher.rs"

[[bin]]
name = "minimal_subscriber"
path = "src/minimal_subscriber.rs"

[dependencies]
rclrs = "0.7"
std_msgs = "*"
geometry_msgs = "*"
visualization_msgs = "*"
builtin_interfaces = "*"
nalgebra = "0.33"
```

### `package.xml`

```xml
<?xml version="1.0"?>
<package format="3">
  <name>two_drones_rust</name>
  <version>0.1.0</version>
  <description>Rust ROS 2 version of VNAV Lab 2.</description>
  <maintainer email="you@example.com">Your Name</maintainer>
  <license>MIT</license>

  <depend>rclrs</depend>
  <depend>std_msgs</depend>
  <depend>geometry_msgs</depend>
  <depend>visualization_msgs</depend>
  <depend>builtin_interfaces</depend>

  <export>
    <build_type>ament_cargo</build_type>
  </export>
</package>
```

Build it:

```bash
cd ~/vnav_rust_ws
colcon build --symlink-install --packages-select two_drones_rust
source install/setup.bash
```

---

## Minimal Rust publisher and subscriber

Before writing the drone transform code, create a tiny Rust publisher/subscriber pair.

### `src/minimal_publisher.rs`

```rust
use std::time::Duration;

use rclrs::{Context, SpinOptions};
use std_msgs::msg::String as StringMsg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("minimal_rust_publisher")?;

    let publisher = node.create_publisher::<StringMsg>("rust_chatter")?;

    let mut count: u64 = 0;
    let _timer = node.create_wall_timer(Duration::from_millis(1000), move || {
        let msg = StringMsg {
            data: format!("Hello from ROS 2 Rust: {count}"),
        };
        count += 1;
        publisher.publish(&msg).expect("failed to publish");
    })?;

    executor.spin(SpinOptions::default()).first_error()?;
    Ok(())
}
```

### `src/minimal_subscriber.rs`

```rust
use rclrs::{Context, SpinOptions};
use std_msgs::msg::String as StringMsg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("minimal_rust_subscriber")?;

    let _subscription = node.create_subscription("rust_chatter", |msg: StringMsg| {
        println!("I heard: {}", msg.data);
    })?;

    executor.spin(SpinOptions::default()).first_error()?;
    Ok(())
}
```

Run them in two terminals:

```bash
ros2 run two_drones_rust minimal_publisher
```

```bash
ros2 run two_drones_rust minimal_subscriber
```

Also inspect them:

```bash
ros2 node list
ros2 topic list
ros2 topic echo /rust_chatter
rqt_graph
```

---

## Launch files

Create `launch/minimal_pub_sub.launch.yaml`:

```yaml
launch:
- node:
    pkg: two_drones_rust
    exec: minimal_publisher
    name: rust_pub

- node:
    pkg: two_drones_rust
    exec: minimal_subscriber
    name: rust_sub
```

Run:

```bash
ros2 launch two_drones_rust minimal_pub_sub.launch.yaml
```

Launch files are useful because most robotic systems require multiple nodes to start together with consistent names, parameters, namespaces, and topic remappings.

---

## Transforms and `tf`

In ROS 2, transforms describe the pose of one coordinate frame relative to another. The standard message is:

```bash
ros2 interface show geometry_msgs/msg/TransformStamped
```

A transform has:

- `header.frame_id`: parent frame,
- `child_frame_id`: child frame,
- translation vector,
- rotation quaternion.

For this lab, we will publish transforms to `/tf` using `tf2_msgs/msg/TFMessage`, which contains a list of `geometry_msgs/msg/TransformStamped` messages.

Useful tools:

```bash
ros2 topic echo /tf
ros2 run tf2_ros tf2_echo world av1
ros2 run tf2_ros tf2_echo av1 av2
ros2 run rqt_tf_tree rqt_tf_tree
rviz2
```

> Rust note: ROS 2 Rust does not use C++ inheritance. In C++ you often create a class that inherits from `rclcpp::Node`. In Rust, you usually create a node value and hold publishers, subscriptions, timers, and shared state as regular Rust values.

---

## The two-drone scenario

We will model two aerial vehicles, AV1 and AV2.

Frames:

- `world`: inertial/world frame.
- `av1`: body frame of the first aerial vehicle.
- `av2`: body frame of the second aerial vehicle.

Positions in the world frame:

\[
o_1^w(t) =
\begin{bmatrix}
\cos(t) \\
\sin(t) \\
0
\end{bmatrix}
\]

\[
o_2^w(t) =
\begin{bmatrix}
\sin(t) \\
0 \\
\cos(2t)
\end{bmatrix}
\]

Orientations:

- AV1 has roll = 0, pitch = 0, yaw = \(t\). Equivalently, it rotates about the world/body z-axis as it moves around the circle.
- AV2 has no rotation relative to the world frame. Its axes remain parallel to the world axes.

The main coding task is to publish:

- `world -> av1`
- `world -> av2`

at 50 Hz.

---

## Deliverables

### Deliverable 1 — Nodes, topics, and launch files

Run a static two-drone scene and answer:

1. What nodes are running?
2. What topics are published?
3. Which topics are consumed by RViz?
4. Which node publishes the transform frames?
5. How would you start the same system manually using `ros2 run` instead of `ros2 launch`?
6. What changes between a static scenario and a moving scenario?

Useful commands:

```bash
ros2 node list
ros2 topic list
ros2 node info /node_name
ros2 topic info /topic_name
rqt_graph
rviz2
```

For your Rust version, create a launch file named `two_drones.launch.yaml` that starts:

- `frames_publisher_node`
- `plots_publisher_node`
- `rviz2` with a saved config file

Example skeleton:

```yaml
launch:
- node:
    pkg: two_drones_rust
    exec: frames_publisher_node
    name: frames_publisher_node

- node:
    pkg: two_drones_rust
    exec: plots_publisher_node
    name: plots_publisher_node

- node:
    pkg: rviz2
    exec: rviz2
    name: rviz2
    args: "-d $(find-pkg-share two_drones_rust)/config/default.rviz"
```

---

### Deliverable 2 — Publish transforms in Rust

Create `src/frames_publisher_node.rs`.

Your node should:

1. Create a ROS 2 node called `frames_publisher_node`.
2. Create a publisher on `/tf`.
3. Create a 50 Hz timer.
4. On each timer tick, compute `t` from a start time.
5. Populate `world -> av1` and `world -> av2` transforms.
6. Publish both transforms as a `tf2_msgs/msg/TFMessage`.

Add `tf2_msgs` to `Cargo.toml` and `package.xml` if your workspace provides it:

```toml
tf2_msgs = "*"
```

```xml
<depend>tf2_msgs</depend>
```

#### Helper math

AV1 translation:

```rust
let x1 = t.cos();
let y1 = t.sin();
let z1 = 0.0;
```

AV2 translation:

```rust
let x2 = t.sin();
let y2 = 0.0;
let z2 = (2.0 * t).cos();
```

Yaw-only quaternion for AV1:

```rust
let half_yaw = 0.5 * t;
let qz = half_yaw.sin();
let qw = half_yaw.cos();
```

AV2 identity quaternion:

```rust
let qx = 0.0;
let qy = 0.0;
let qz = 0.0;
let qw = 1.0;
```

#### Rust implementation skeleton

```rust
use std::time::{Duration, Instant};

use builtin_interfaces::msg::Time;
use geometry_msgs::msg::TransformStamped;
use rclrs::{Context, SpinOptions};
use tf2_msgs::msg::TFMessage;

fn now_msg(start: Instant) -> Time {
    let elapsed = start.elapsed();
    Time {
        sec: elapsed.as_secs() as i32,
        nanosec: elapsed.subsec_nanos(),
    }
}

fn make_transform(
    stamp: Time,
    parent: &str,
    child: &str,
    translation: [f64; 3],
    quaternion_xyzw: [f64; 4],
) -> TransformStamped {
    let mut tf = TransformStamped::default();
    tf.header.stamp = stamp;
    tf.header.frame_id = parent.to_string();
    tf.child_frame_id = child.to_string();

    tf.transform.translation.x = translation[0];
    tf.transform.translation.y = translation[1];
    tf.transform.translation.z = translation[2];

    tf.transform.rotation.x = quaternion_xyzw[0];
    tf.transform.rotation.y = quaternion_xyzw[1];
    tf.transform.rotation.z = quaternion_xyzw[2];
    tf.transform.rotation.w = quaternion_xyzw[3];

    tf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("frames_publisher_node")?;

    let tf_pub = node.create_publisher::<TFMessage>("/tf")?;
    let start = Instant::now();

    let _timer = node.create_wall_timer(Duration::from_millis(20), move || {
        let t = start.elapsed().as_secs_f64();
        let stamp = now_msg(start);

        let av1_translation = [t.cos(), t.sin(), 0.0];
        let av2_translation = [t.sin(), 0.0, (2.0 * t).cos()];

        let half_yaw = 0.5 * t;
        let av1_quat = [0.0, 0.0, half_yaw.sin(), half_yaw.cos()];
        let av2_quat = [0.0, 0.0, 0.0, 1.0];

        let world_t_av1 = make_transform(
            stamp.clone(),
            "world",
            "av1",
            av1_translation,
            av1_quat,
        );

        let world_t_av2 = make_transform(
            stamp,
            "world",
            "av2",
            av2_translation,
            av2_quat,
        );

        let msg = TFMessage {
            transforms: vec![world_t_av1, world_t_av2],
        };

        tf_pub.publish(&msg).expect("failed to publish /tf");
    })?;

    executor.spin(SpinOptions::default()).first_error()?;
    Ok(())
}
```

#### How to test

```bash
cd ~/vnav_rust_ws
colcon build --symlink-install --packages-select two_drones_rust
source install/setup.bash
ros2 run two_drones_rust frames_publisher_node
```

In another terminal:

```bash
ros2 topic echo /tf
ros2 run tf2_ros tf2_echo world av1
ros2 run tf2_ros tf2_echo world av2
ros2 run rqt_tf_tree rqt_tf_tree
rviz2
```

Expected result:

- `av1` moves on a circle in the world x-y plane.
- `av2` moves in the world x-z plane.
- `av1` rotates with yaw equal to `t`.
- `av2` does not rotate.

---

### Deliverable 3 — Look up a relative transform

The original lab asks you to use tf to populate a transform between two frame names, `ref_frame` and `dest_frame`.

In Rust, you have two reasonable options.

#### Option A — Use ROS tools externally

Use your Rust node to publish `/tf`, then use existing ROS 2 tools to query:

```bash
ros2 run tf2_ros tf2_echo av1 av2
```

This verifies the transform tree and gives you the relative transform from `av1` to `av2`.

#### Option B — Implement the relative transform math in Rust

Because the transforms in this lab are known analytically, you can compute:

\[
{}^{av1}T_{av2}(t) = ({}^wT_{av1}(t))^{-1} {}^wT_{av2}(t)
\]

This is a good Rust math exercise and avoids waiting on higher-level `tf2` Rust ergonomics.

Create `src/transform_math.rs` or implement this inside `plots_publisher_node.rs` using `nalgebra`.

```rust
use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};

fn world_t_av1(t: f64) -> Isometry3<f64> {
    let trans = Translation3::new(t.cos(), t.sin(), 0.0);
    let rot = UnitQuaternion::from_euler_angles(0.0, 0.0, t);
    Isometry3::from_parts(trans, rot)
}

fn world_t_av2(t: f64) -> Isometry3<f64> {
    let trans = Translation3::new(t.sin(), 0.0, (2.0 * t).cos());
    let rot = UnitQuaternion::identity();
    Isometry3::from_parts(trans, rot)
}

fn av1_t_av2(t: f64) -> Isometry3<f64> {
    world_t_av1(t).inverse() * world_t_av2(t)
}

fn av2_origin_in_av1(t: f64) -> Vector3<f64> {
    av1_t_av2(t).translation.vector
}
```

Now publish the relative point as a visualization marker, path, or simple `geometry_msgs/msg/PointStamped`.

Suggested topic names:

- `/av1_path_world`
- `/av2_path_world`
- `/av2_path_av1`

Expected result:

- AV1 trajectory in world: circle.
- AV2 trajectory in world: parabola-like curve in the x-z plane.
- AV2 trajectory relative to AV1: an ellipse on a slanted plane.

---

### Deliverable 4 — Mathematical derivations

Write up your derivation in Markdown or LaTeX.

#### 1. Show AV2 follows a parabola-like curve in the world x-z plane

Given:

\[
x = \sin(t), \quad y = 0, \quad z = \cos(2t)
\]

Use:

\[
\cos(2t) = 1 - 2\sin^2(t)
\]

Therefore:

\[
z = 1 - 2x^2
\]

So the trajectory lies in the plane \(y = 0\) and follows a parabola in the \(x\)-\(z\) coordinates.

#### 2. Compute \(o_2^1(t)\)

You need:

\[
{}^wT_1(t) =
\begin{bmatrix}
{}^wR_1(t) & o_1^w(t) \\
0 & 1
\end{bmatrix}
\]

\[
{}^wT_2(t) =
\begin{bmatrix}
I & o_2^w(t) \\
0 & 1
\end{bmatrix}
\]

Then:

\[
{}^1T_2(t) = ({}^wT_1(t))^{-1} {}^wT_2(t)
\]

and the position of AV2 in AV1's frame is:

\[
o_2^1(t) = ({}^wR_1(t))^T (o_2^w(t) - o_1^w(t))
\]

Since AV1 has yaw \(t\):

\[
{}^wR_1(t) =
\begin{bmatrix}
\cos t & -\sin t & 0 \\
\sin t & \cos t & 0 \\
0 & 0 & 1
\end{bmatrix}
\]

So:

\[
o_2^1(t) =
\begin{bmatrix}
\cos t & \sin t & 0 \\
-\sin t & \cos t & 0 \\
0 & 0 & 1
\end{bmatrix}
\left(
\begin{bmatrix}
\sin t \\
0 \\
\cos 2t
\end{bmatrix}
-
\begin{bmatrix}
\cos t \\
\sin t \\
0
\end{bmatrix}
\right)
\]

You should simplify this expression fully in your writeup.

#### 3. Show that \(o_2^1(t)\) is planar

After simplifying, find a linear relation between the components of \(o_2^1(t)\). In particular, look for a relation between the y and z components.

Your answer should have the form:

\[
a x_2^1 + b y_2^1 + c z_2^1 + d = 0
\]

or a simpler relation if one component is not needed.

#### 4. Re-express the curve in a 2D frame on the plane

Define a new 2D coordinate frame on the plane of the relative trajectory.

The original lab suggests centering the frame at:

\[
p^1 = (-1, -1/2, 0)
\]

Your goal is to choose axes \((x_p, y_p)\) such that:

- the curve is centered at the origin of the plane frame,
- the axes align with the ellipse's symmetry axes,
- the out-of-plane coordinate vanishes.

#### 5. Show the relative trajectory is an ellipse

Once you have \(o_2^p(t)\), show that it satisfies the axis-aligned ellipse equation:

\[
\frac{x_p^2}{a^2} + \frac{y_p^2}{b^2} = 1
\]

Compute the semi-axis lengths \(a\) and \(b\).

---

### Deliverable 5 — Quaternion properties

Let:

\[
\Omega_1(q) =
\begin{bmatrix}
q_4 & -q_3 & q_2 & q_1 \\
q_3 & q_4 & -q_1 & q_2 \\
-q_2 & q_1 & q_4 & q_3 \\
-q_1 & -q_2 & -q_3 & q_4
\end{bmatrix}
\]

\[
\Omega_2(q) =
\begin{bmatrix}
q_4 & q_3 & -q_2 & q_1 \\
-q_3 & q_4 & q_1 & q_2 \\
q_2 & -q_1 & q_4 & q_3 \\
-q_1 & -q_2 & -q_3 & q_4
\end{bmatrix}
\]

For unit quaternions:

\[
q_a \otimes q_b = \Omega_1(q_a)q_b = \Omega_2(q_b)q_a
\]

Prove:

1. If \(q\) is a unit quaternion, then both \(\Omega_1(q)\) and \(\Omega_2(q)\) are orthogonal matrices.
2. For a unit quaternion \(q\):

\[
\Omega_1(q)^T q = \Omega_2(q)^T q = [0,0,0,1]^T
\]

3. For any \(x,y \in \mathbb{R}^4\):

\[
\Omega_1(x)\Omega_2(y) = \Omega_2(y)\Omega_1(x)
\]

and:

\[
\Omega_1(x)\Omega_2(y)^T = \Omega_2(y)^T\Omega_1(x)
\]

---

### Optional Deliverable 6 — Intrinsic vs extrinsic rotations

Consider the rotations:

- \(R_0\): 90° around x
- \(R_1\): 180° around y
- \(R_2\): -30° around x

Show that an extrinsic sequence about fixed/world axes can produce the same final orientation as an intrinsic sequence about body/local axes when the order is reversed.

Hint:

- A rotation about a world axis is applied by pre-multiplication.
- A rotation about a body axis is applied by post-multiplication.
- Use this to prove the equivalence for a general sequence.

---

## Suggested repository layout

```text
two_drones_rust/
├── Cargo.toml
├── package.xml
├── README.md
├── config/
│   └── default.rviz
├── launch/
│   ├── minimal_pub_sub.launch.yaml
│   └── two_drones.launch.yaml
├── mesh/
│   └── quadrotor.dae
└── src/
    ├── minimal_publisher.rs
    ├── minimal_subscriber.rs
    ├── frames_publisher_node.rs
    ├── plots_publisher_node.rs
    └── transform_math.rs
```

---

## Debugging checklist

### Build problems

```bash
source /opt/ros/$ROS_DISTRO/setup.bash
source ~/vnav_rust_ws/install/setup.bash
colcon build --symlink-install
```

If Rust packages are ignored by `colcon`, verify:

```bash
pip show colcon-cargo
pip show colcon-ros-cargo
```

If message crates are missing, check that the corresponding ROS interface repository is in `src/` and that `package.xml` lists the dependency.

---

### Runtime problems

Check whether the node exists:

```bash
ros2 node list
```

Check whether `/tf` exists:

```bash
ros2 topic list | grep tf
```

Check whether transforms are valid:

```bash
ros2 run tf2_ros tf2_echo world av1
ros2 run tf2_ros tf2_echo world av2
ros2 run tf2_ros tf2_echo av1 av2
```

Open the frame tree:

```bash
ros2 run rqt_tf_tree rqt_tf_tree
```

Open RViz:

```bash
rviz2
```

Set the fixed frame to `world`, then try `av1`.

---

## Where to go next

After completing this Rust version of Lab 2, good follow-up projects are:

1. Add unit tests for all SE(3) operations using `nalgebra`.
2. Implement your own mini `TfBuffer` in Rust.
3. Record the `/tf` stream to a rosbag and replay it.
4. Add `/visualization_marker_array` paths for AV1, AV2, and AV2-in-AV1.
5. Extend the lab into a connectivity-aware telemetry demo: buffer the transform stream locally and upload logs only when a simulated network connection is available.

---

## Submission checklist

Submit:

- `two_drones_rust/` source package.
- `README.md` explaining how to build and run.
- Screenshots of:
  - `rqt_graph`,
  - `rqt_tf_tree`,
  - RViz with fixed frame `world`,
  - RViz with fixed frame `av1`.
- A short math PDF or Markdown writeup for Deliverables 4–6.
- A note describing which ROS 2 distro and `rclrs` version you used.
