## Web Crawler

This is a simple web crawler that crawls the top posts from hacker news and stores them in a Postgres database. It also provides a simple API to query the database.


### Requirements

- Rust
- Docker

### Usage

```bash
$ cargo run --release
```

### API
The API is available at `http://localhost:3000`. The following endpoints are available:

- `/posts/<id>` - Returns a post by id
- `/posts` - Returns all posts
- `/posts/top` - Returns the top 10 posts ordered by score
- `/posts/<user>` - Returns all posts by a user

#### Example
```bash
$ curl http://localhost:3000/posts/top
```


### Install and Run Postgres
The easist way to run Postgres is using a container. The `docker-compose.yml` file contains the configuration to run Postgres in a container. To run Postgres, run the following command:

```bash
$ docker-compose up -d
```

To access the Postgres database, run the following command:

```bash
$ docker exec -it crawler-rs-db-1 psql -d db -U user
```

To print the top 10 posts, run the following command:

```bash
$ docker exec -it crawler-rs-db-1 psql -d db -U user -c "SELECT * FROM posts ORDER BY score DESC LIMIT 10"
```

### TODOs
- Add tests (unit and integration tests)
