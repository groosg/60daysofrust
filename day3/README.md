# Day 3 - 60 Days of Rust

Today I spent some time reading through the [actix-web documentation](https://actix.rs/docs) and implemented the guessing game (see [day 2](../day2/)) via HTTP.

This is a dummy implementation that accepts raw values from a post request and compares with a fixed random number returning some text.

Moving forward I want to find some time to implement something closer to an API with more robust serialization, error handling, etc.

## How to play

Start the server

```shell
cd day3
cargo run
```

In a different terminal session use `curl` or any other tool to make requests.

```shell
curl -L -d 55 -X POST http://localhost:8080/guess
Too big!

curl -L -d 45 -X POST http://localhost:8080/guess
Too small!

curl -L -d 50 -X POST http://localhost:8080/guess
You win!
```

