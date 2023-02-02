# Web Crawler

This is a simple web crawler that crawls the top posts from hacker news and stores them in a Postgres database. It also provides a simple API to query the database.


## Requirements

- Rust
- Docker

## Usage

### Install and Run Postgres

The easist way to run Postgres is using a container. The `docker-compose.yml` file contains the configuration to run Postgres in a container. To run Postgres, run the following command:

```bash
docker-compose up -d
```

To access the Postgres database, run the following command:

```bash
docker exec -it <container-name> psql -d db -U user
```

You can use docker to directly run the SQL commands. For example, to print the top 10 posts, run the following command:

```bash
docker exec -it <container-name> psql -d db -U user -c "SELECT * FROM posts ORDER BY score DESC LIMIT 10"
```

### Build and Run the Crawler

Before running the crawler, make sure that Postgres is running. To build and run the crawler, run the following command:

```bash
cargo run --release
```

## API
The API is available at `http://localhost:3000`. The following endpoints are available:

- `/posts/<id>` - Returns a post by id
- `/posts` - Returns all posts
- `/posts/top` - Returns the top 10 posts ordered by score
- `/posts/<user>` - Returns all posts by a user

### Example

To get the top 10 posts, run the following command:
```bash
curl http://localhost:3000/posts/top | jq
```

To get the posts of a user, run the following command:
```bash
curl 127.0.0.1:3000/posts/ValentineC | jq
```

> jq is a command line JSON processor that can be used to format the JSON output.

## TODOs
- Add tests (unit and integration tests)
