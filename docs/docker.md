# Run the database shell with Docker

With Docker, you can easily get started and experiment with the database system. All database files and logs will remain in the container. If you want to persist the created database files, you need to use volumes.

To build the image:

```bash
cd database-system
```
```docker
docker build -t dbsystem-shell .
```

Run a container:
```docker
docker run --rm -it dbsystem-shell
```