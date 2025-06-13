# 🦀 Task Tracker API (Rust + Axum)

A simple and modular **Task Tracker API** built using **Rust** and **Axum**. This project demonstrates how to create a RESTful API with in-memory state management and support for basic CRUD operations.

---

## 🚀 Features

- ✅ Health Check (`GET /health`)
- 📄 Get All Tasks (`GET /tasks`)
- 🔍 Get Task by ID (`GET /tasks/:id`)
- 🆕 Create Task (`POST /tasks`)
- 📝 Update Task (`PUT /tasks/:id`)
- ❌ Delete Task (`DELETE /tasks/:id`)
- 🔄 Toggle Completion Status (`PATCH /tasks/:id/toggle`)

---

## 🧩 Tech Stack

- **Language:** Rust
- **Framework:** [Axum](https://docs.rs/axum/latest/axum/)
- **State Management:** `Arc<Mutex<Vec<Task>>>`
- **UUID Support:** `uuid` crate
- **Serialization:** `serde`

---

## 🛠️ Setup

### 1. Clone the repo

```bash
git clone https://github.com/yourusername/task_tracker_api.git
cd task_tracker_api
```

### 2. Install dependencies

Make sure you have Rust installed: https://rustup.rs

Then run:

```bash
cargo build
```

### 3. Run the server

```bash
cargo run
```

Visit: `http://localhost:3000`

---

## 📬 API Endpoints

### Health Check
```http
GET /health
```

### Get All Tasks
```http
GET /tasks
```

### Get Task by ID
```http
GET /tasks/{id}
```

### Create New Task
```http
POST /tasks
Content-Type: application/json

{
  "title": "Learn Axum"
}
```

### Update Task
```http
PUT /tasks/{id}
Content-Type: application/json

{
  "id": "UUID_HERE",
  "title": "Updated Title",
  "completed": true
}
```

### Delete Task
```http
DELETE /tasks/{id}
```

### Toggle Task Completion
```http
PATCH /tasks/{id}/toggle
```

---

## 🧠 Project Structure

```
src/
├── main.rs        # App entrypoint & router setup
├── handlers.rs    # Route handler logic
├── models.rs      # Task structs & types
└── state.rs       # Shared application state
```
