# Run the database shell with Docker

With Docker, you can easily get started and experiment with the database system. All database files and logs will remain in the container. If you want to persist the created database files, you need to use volumes.

To build the image

```bash
cd database-system
```
```bash
docker build -t dbsystem-shell .
```

Run a container

```bash
docker run --rm -it dbsystem-shell
```

# Run with Docker Compose

Alternatively, you can run the database shell with Docker Compose. The `docker-compose.yml` is found in the project root.

The Docker Compose uses two volumes. One for database directory and one for logs directory. With these, databases and logs are persisted.

Run the shell service interactively and remove the container when exited

```bash
cd database-system
```
```bash
docker compose run --rm shell
```

To verify the volumes were created, you can run

```bash
docker volume ls
```