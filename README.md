# Django vs Rust Axum — Todo API Benchmark

A side-by-side comparison of the same Todo REST API implemented in **Django (Python)** and **Axum (Rust)**, designed for benchmarking performance differences.

Both servers expose identical CRUD endpoints backed by SQLite.

## Project Structure

```
django_rust/
├── todo_django/       # Django + DRF implementation (port 8000)
├── todo_rust/         # Axum + SQLx implementation (port 8001)
└── README.md
```

## Prerequisites

- **Python 3.10+** with `django` and `djangorestframework`
- **Rust 1.70+** with `cargo`
- **curl** or **Postman** for manual testing
- **hey**, **wrk**, or **ab** for load testing

## Starting the Servers

### Django (port 8000)

```bash
cd todo_django
pip install django djangorestframework
python manage.py migrate
python manage.py runserver 8000
```

### Rust Axum (port 8001)

```bash
cd todo_rust
cargo run --release
```

> The Rust server auto-creates the SQLite database and table on startup.

## API Endpoints

| Method | Path              | Description    |
|--------|-------------------|----------------|
| GET    | `/api/todos`      | List all todos |
| POST   | `/api/todos`      | Create a todo  |
| GET    | `/api/todos/{id}` | Get one todo   |
| PUT    | `/api/todos/{id}` | Full update    |
| PATCH  | `/api/todos/{id}` | Partial update |
| DELETE | `/api/todos/{id}` | Delete a todo  |

> Django uses a trailing slash (`/api/todos/`), Rust does not (`/api/todos`).

## Running Benchmarks

### Install a Load Testing Tool

Pick one of the following:

```bash
# hey (recommended, Go-based)
# https://github.com/rakyll/hey
go install github.com/rakyll/hey@latest

# wrk (Linux/macOS)
# https://github.com/wg/wrk
brew install wrk        # macOS
sudo apt install wrk    # Ubuntu

# ab (Apache Bench, comes with Apache or httpd-tools)
sudo apt install apache2-utils   # Ubuntu
```

### Seed Test Data

Before benchmarking read-heavy endpoints, seed both databases with sample data:

```bash
# Django
for i in $(seq 1 100); do
  curl -s -X POST http://127.0.0.1:8000/api/todos/ \
    -H "Content-Type: application/json" \
    -d "{\"title\": \"Task $i\", \"description\": \"Description for task $i\"}" > /dev/null
done

# Rust
for i in $(seq 1 100); do
  curl -s -X POST http://127.0.0.1:8001/api/todos \
    -H "Content-Type: application/json" \
    -d "{\"title\": \"Task $i\", \"description\": \"Description for task $i\"}" > /dev/null
done
```

### Benchmark: List Todos (GET)

```bash
# Django
hey -n 5000 -c 50 http://127.0.0.1:8000/api/todos/

# Rust
hey -n 5000 -c 50 http://127.0.0.1:8001/api/todos
```

### Benchmark: Create Todo (POST)

```bash
# Django
hey -n 5000 -c 50 -m POST \
  -H "Content-Type: application/json" \
  -d '{"title": "Benchmark task", "description": "test"}' \
  http://127.0.0.1:8000/api/todos/

# Rust
hey -n 5000 -c 50 -m POST \
  -H "Content-Type: application/json" \
  -d '{"title": "Benchmark task", "description": "test"}' \
  http://127.0.0.1:8001/api/todos
```

### Benchmark: Get Single Todo (GET by ID)

```bash
# Django
hey -n 5000 -c 50 http://127.0.0.1:8000/api/todos/1/

# Rust
hey -n 5000 -c 50 http://127.0.0.1:8001/api/todos/1
```

### Using wrk (alternative)

```bash
# Django — 10 seconds, 4 threads, 50 connections
wrk -t4 -c50 -d10s http://127.0.0.1:8000/api/todos/

# Rust
wrk -t4 -c50 -d10s http://127.0.0.1:8001/api/todos
```

## Understanding the Results

### Key Metrics to Compare

| Metric                  | What It Tells You                                      |
|-------------------------|--------------------------------------------------------|
| **Requests/sec**        | Throughput — how many requests the server can handle   |
| **Avg latency**         | Mean response time per request                         |
| **P99 latency**         | Worst-case tail latency (99th percentile)              |
| **Max latency**         | Single worst response time                             |
| **Transfer/sec**        | Raw data throughput                                    |
| **Error rate**          | Percentage of non-2xx responses under load             |

### Expected Benchmark Characteristics

| Area               | Django                              | Rust Axum                              |
|--------------------|-------------------------------------|----------------------------------------|
| **Throughput**     | Lower — Python GIL, interpreter overhead | Higher — compiled, async, zero-cost abstractions |
| **Latency**       | Higher avg and tail latency         | Lower and more consistent              |
| **Memory usage**   | Higher baseline (~30-50 MB)         | Lower baseline (~5-10 MB)              |
| **Cold start**     | Faster (no compilation)             | Slower first build, instant thereafter |
| **Concurrency**    | Limited by GIL (1 thread for Python code) | Native async with multi-threaded tokio runtime |
| **Dev speed**      | Faster iteration, batteries included | Slower iteration, more boilerplate     |

### Tips for Fair Comparison

1. **Use release mode for Rust** — `cargo run --release` vs debug makes a 5-10x difference.
2. **Run Django with gunicorn** for a production-like setup:
   ```bash
   pip install gunicorn
   cd todo_django
   gunicorn config.wsgi:application -w 4 -b 127.0.0.1:8000
   ```
3. **Warm up both servers** with a few hundred requests before measuring.
4. **Run benchmarks multiple times** and take the median.
5. **Monitor system resources** during the test:
   ```bash
   # CPU and memory per process
   # Linux/macOS
   top -p $(pgrep -f "manage.py\|todo_rust")

   # Windows
   tasklist /FI "IMAGENAME eq python.exe"
   tasklist /FI "IMAGENAME eq todo_rust.exe"
   ```
6. **Keep the database size identical** across both servers before each test run.
