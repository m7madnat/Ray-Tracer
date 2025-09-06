# Ray-Tracer

A simple and efficient raytracer written in Rust. This project renders 3D models using customizable camera positions, lighting, and shading. Output images are generated in PPM format and can be easily viewed with any image viewer.

---

## ✨ Features

- Render spheres, cubes, cylinders, planes, and more
- Adjustable camera position and orientation
- Configurable brightness and lighting
- Multiple example scenes included
- Fast rendering with Rust's performance
- Easy-to-edit source code for custom scenes

---

## 🛠 Tech Stack

- **Language:** Rust
- **Build System:** Cargo
- **Image Output:** PPM format
- **Dependencies:** None (pure Rust implementation)

---

## 🚀 Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/ray-tracer.git
   cd ray-tracer
   ```

2. **Build the project (release mode recommended):**
   ```bash
   cargo build --release
   ```

---

## 📦 Usage

### Render an Example Scene

Run the raytracer with the desired object name:

```bash
cargo run --release <object_name>
```

**Example:**
```bash
cargo run --release cube
```

This will render a cube scene using the camera settings in [`src/camera.rs`](src/camera.rs).

### Camera Control

Camera settings are hardcoded in [`src/camera.rs`](src/camera.rs):

```rust
let look_from = Vec3::new(...);  // Camera position
let look_at = Vec3::new(...);    // Where the camera looks
```

Edit these values in the `Camera::new()` method for different views (top, side, front).

### Brightness & Lighting

Brightness and shading are affected by light direction and dot product logic. You can tweak this inside [`src/main.rs`](src/main.rs):

```rust
color = (color / (samples_per_pixel as f32)) * 1.5;
```

Increase `1.5` for brighter output, decrease for darker.

### Output

After rendering, the image is saved as:

```
output.ppm
```

Open `output.ppm` with any image viewer that supports PPM format.

---

## 🧩 Contribution Guidelines

1. Fork the repository and create your branch.
2. Make your changes with clear commit messages.
3. Submit a pull request describing your changes.

All contributions are welcome! Please follow Rust best practices and ensure code is well-documented.

---

## 📄 License

This project is licensed under the [LICENSE](LICENSE) (placeholder).

---