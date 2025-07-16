# ⚙️ Algonaut



https://github.com/user-attachments/assets/fd37a4dc-ffb9-44e8-b0a1-7b2a1a97c62c



**Algonaut** is an on-demand, container-based **code execution editor** platform. Each editor session is called a **cube**, which runs in its own isolated Docker container, offering live sockets, persistent storage, and language-specific runtime environments.

> Think: spin up a coding sandbox only when needed, isolate it, sync via sockets, and kill it when done — all using Redis and Rust.

---

## 🧩 Key Concepts

### 🧱 Cube
- A **Cube** is a lightweight, ephemeral code editor environment.
- Each cube is represented as a standalone container.
- Exposes two ports:
  - 🚪 Port 1: REST API for project/file operations
  - 📡 Port 2: WebSocket for live sync and execution

### ⚙️ Container Orchestration
- A Rust-based orchestrator continuously polls a Redis queue (`execution`)
- New jobs are pulled every 10 seconds
- Uses CLI-based Docker commands to:
  - Spin up containers on-demand
  - Attach containers to a Docker network
  - Rehydrate containers with previous session data
  - Gracefully stop & remove containers after execution
- On teardown, the container file system is uploaded to persistent storage

---

## 🌐 Tech Stack

| Layer               | Tech                             |
|---------------------|----------------------------------|
| Orchestration       | **Rust** (`tokio`, `redis`, `dotenvy`) |
| Runtime Execution   | **Node.js** / TypeScript-based containers |
| Message Queue       | **Redis**                        |
| Persistence         | **S3-compatible** backend  |
| Containerization    | **Docker**                       |

---

## 📦 Redis Job Payload Format

To trigger a cube, push a job into the Redis queue `execution`:

```json
{
  "id": "cube_abc123",
  "user_id": "user_xyz",
  "name": "PythonSandbox",
  "cube_type": "python"
}
```

---

## 🧪 Example Flow

1. 🧠 Frontend sends a request to launch a cube
2. 📨 Backend pushes a job to Redis (`execution`)
3. ⚙️ Algonaut orchestrator (Rust) pulls job from queue
4. 🐳 Docker container is created and attached to network
5. 🌐 Cube becomes accessible on allocated ports
6. 💾 On shutdown, filesystem is backed up to S3


---

## 🧬 Environment Variables

Create a `.env` file at the root:

```env
REDIS_URL=redis://localhost/
DOCKER_USER_CONTAINER_IMAGE=algonaut-runtime-node
DOCKER_NETWORK=algonaut-network
```

---

## 🐳 Runtime Container (in `/container`)

Each cube is backed by a runtime container which:

- Exposes a REST API for:
  - `GET /project` → returns file system
  - `PUT /reinit` → restores prior session data
- Optionally hosts a WebSocket server for real-time collaboration or execution triggers

> The container is designed to be language-agnostic and scriptable for multiple runtimes (Python, JS, Go, etc.)

---

## 🚦 Running the Orchestrator

```bash
# build
cargo build --release

# run orchestrator
cargo run
```

This will start polling Redis every 10 seconds and orchestrate containers dynamically.

---

## 🧰 Dev Notes

- Supports hot container recovery and backup
- Modular design (easily pluggable runtimes)
- Easy to deploy with Docker Compose
- Can be integrated into interview platforms, learning environments, or cloud IDEs

---

## 📌 Status

- ✅ Redis-based job queue
- ✅ Docker CLI orchestration
- ✅ REST & WebSocket runtime support
- ✅ Persistent session backup (S3)
- 🔲 Language runtime abstraction
- 🔲 Collaboration engine (Yjs or CRDT)
- 🔲 Web dashboard for admins

---

## 📜 License

MIT License.  
Built with 🦀 and ☕ by [Your Name]

---

## 📣 Contributions

Open to PRs, suggestions, and extensions. You can help by:

- Building runtime containers for more languages
- Improving performance and sandboxing
- Adding devtools and monitoring dashboards
