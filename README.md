# aoj-icpc-v2
AOJ ICPC v2

### Set up dev environment
Install [docker-compose](https://docs.docker.com/compose/install/) first.

Then, this command will set up containers for dev environment.
```
$ docker-compose up -d
```
Try to connect to [http://localhost:5050/](http://localhost:5050/) after DB set up.

You can stop containers and remove volumes and start again if you have trouble.
```
$ docker-compose down -v
```

### DB set up
Initially, you need DB scheme change and initialize with dummy data.
```
$ docker-compose run api diesel migration run
$ docker-compose run api cargo run --bin initialize_db
```
