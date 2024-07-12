# Url Shortener

A URL shortener that you can use from your terminal - built with cyndra, rocket and postgres/sqlx.

## How to use it

You can use this URL shortener directly from your terminal. Just copy and paste this command to your terminal and replace `<URL>` with the URL that you want to shorten

```bash
curl -X POST -d '<URL>' https://s.cyndraapp.rs
```

like this

```bash
curl -X POST -d 'https://docs.rs/cyndra-service/latest/cyndra_service/' https://s.cyndraapp.rs
```

you will get the shortened URL back (something like this `https://s.cyndraapp.rs/RvpVU_`)

## Project structure

The project consists of the following files

- `Cyndra.toml` contains the name of the app (if name is `s` domain will be `s.cyndraapp.rs`)
- `migrations` folder is for DB migration files created by [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)
- `src/lib.rs` is where all the magic happens - it creates a cyndra service with two endpoints: one for creating new short URLs and one for handling shortened URLs.

## How to deploy

To deploy this app, check out the repository locally

```bash
$ git clone https://github.com/getsynth/cyndra.git
```

navigate to `examples/url-shortener`

```bash
$ cd examples/url-shortener
```

install cyndra

```bash
$ cargo install cargo-cyndra
```

login to cyndra

```bash
$ cargo cyndra login
```

open up the `Cyndra.toml` file and change the project name to something 
unique - in cyndra, projects are globally unique. Then run

```bash
$ cargo cyndra deploy
```
