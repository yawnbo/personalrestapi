# DOCS
these are very basic docs that are NOT written by me and instead by an ai with the notes that are currently in the codebase. Please read them and make sure to read the src/mod file and look for any other FIXME tags for issues to be aware of

## Database Support
This project supports both PostgreSQL and SQLite:
- **PostgreSQL**: Use `DATABASE_URL=postgres://...` for production (recommended for Fly.io)
- **SQLite**: Use `DATABASE_URL=sqlite://db.sqlite` for development or fallback

The database type is auto-detected from the connection string prefix.

## Development Setup

### Start PostgreSQL + Redis for local development:
```bash
podman-compose -f docker-compose.test.yml up postgres redis -d
# or with docker:
docker-compose -f docker-compose.test.yml up postgres redis -d
```

### Run the server with PostgreSQL:
```bash
DATABASE_URL=postgresql://testuser:testpass@localhost:5432/testdb \
REDIS_URL=redis://localhost:6379 \
RUN_MIGRATIONS=true \
CARGO_ENV=development \
ACCESS_TOKEN_SECRET=dev_access_secret \
REFRESH_TOKEN_SECRET=dev_refresh_secret \
CORS_ORIGIN=http://localhost:3000 \
PREVIEW_CORS_ORIGIN=http://localhost:3001 \
SEED=false \
cargo run
```

### Connect to PostgreSQL for debugging:
```bash
# With psql
psql postgresql://testuser:testpass@localhost:5432/testdb

# Or connect to running container
podman exec -it back-postgres-1 psql -U testuser -d testdb
```

### Stop services:
```bash
podman-compose -f docker-compose.test.yml down
```

## Running Tests

### With Podman (PostgreSQL + Redis):
```bash
podman-compose -f docker-compose.test.yml up --build
```

### With Docker (PostgreSQL + Redis):
```bash
docker-compose -f docker-compose.test.yml up --build
```

### Locally with PostgreSQL:
```bash
# Start postgres and redis
podman-compose -f docker-compose.test.yml up postgres redis -d

# Run tests
DATABASE_URL=postgresql://testuser:testpass@localhost:5432/testdb \
REDIS_URL=redis://localhost:6379 \
RUN_MIGRATIONS=true \
cargo test
```

### Locally with SQLite:
```bash
DATABASE_URL=sqlite::memory: \
REDIS_URL=redis://localhost:6379 \
RUN_MIGRATIONS=true \
cargo test
```

## API Endpoints

### Streams (Protected)
- `GET /api/v1/streams` - Get all streams/games grouped by category with auto-refresh
  ```json
  {
    "categories": [
      {
        "category": "American Football",
        "games": [
          {
            "id": 12239,
            "name": "Florida Gators vs. Texas A&M Aggies",
            "poster": "https://...",
            "start_time": 1760223600,
            "end_time": 1760236200,
            "cache_time": 1760234070,
            "video_link": "https://...",
            "category": "American Football"
          }
        ]
      }
    ]
  }
  ```
- `GET /api/v1/streams/{provider}` - Get stream for specific provider
- `GET /api/v1/streams/ppvsu/{id}` - Get specific PPVSU game by ID
- `DELETE /api/v1/streams/ppvsu/cache` - Clear PPVSU Redis cache (forces fresh fetch on next request)
