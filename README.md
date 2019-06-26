# aoj-icpc-v2
AOJ ICPC v2

### How to set up dev environment
Install [docker-compose](https://docs.docker.com/compose/install/) first.
Then, this command will set up dev environment.
```
$ docker-compose up -d
```

### DB commands
Initially, you need DB scheme setup
```
$ docker-compose run api diesel migration run
```
