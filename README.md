# GeekApk Rust Server

[![Travis Status](https://img.shields.io/travis/geekapk/GeekApkR.svg?style=flat-square)](https://travis-ci.org/geekapk/GeekApkR)
[![Rocket Powered](https://img.shields.io/badge/powered-rocket-red.svg?style=flat-square)](https://rocket.rs)
[![APIs](https://img.shields.io/badge/doc-apis-yellow.svg?style=flat-square)](API.md)
[![Android UI](https://img.shields.io/badge/doc-frontend-yellow.svg?style=flat-square)](Android_UI.txt)

> Pure android market&community

🔮 [GeekApk](https://geekapk.org) __API Server__ written in [Rust](https://rust-lang.org)

Web framework used: [Rocket](https://rocket.rs)

ORM: [Diesel](https://diesel.rs)

## GeekApk Organization Service

+ [geekapk.org:233](http://geekapk.org.origin_ip:233/) HTTP API
+ [api.geekapk.org](https://api.geekapk.org/) SSL Proxy

## Installation

### Compiling & running

> Get Rust Toolchian at rust-lang.org

```bash
git clone https://github.com/geekapk/GeekApkR.git&&cd GeekApkR
cargo build
```

## Usage

### For Development

> Database config priority: DATABASE_URL, ga_username(=geekapk)&ga_password(=dolphins), geekapk.ini

```bash
export ga_username=#(YOUR PG USER FOR GEEKAPK)
export ga_password=#(PASSWORD FOR GEEKAPK DATABASE USER)
cargo build
./target/debug/geekapkd
```

### [Deploy](DEPLOY.md)

## Development

:heart: This application made use of the [Rocket Framework](https://rocket.rs/) and the [Diesel ORM](https://diesel.rs/)

:full_moon_with_face::+1: Contributions are welcome, check [Android UI](Android_UI.txt) for feature designment.

### Docs

See [GeekApk Dev](https://geekapk.org/dev/) or [GeekApkR Wiki](https://github.com/geekapk/GeekApkR/wiki)

## :rocket: Testing

Prepare for Postgres database:

```sql
CREATE USER geekapk WITH PASSWORD 'dolphins';

CREATE DATABASE geekapk_db;

GRANT ALL PRIVILEGES ON DATABASE geekapk_db TO geekapk;
```

```bash
# Prepare for environment
cargo test
```

## Contributing

1. [Fork it](https://github.com/geekapk/GeekApkR/fork)
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create a new Pull Request

## Contributors

+ [duangsuse](https://github.com/duangsuse) duangsuse - creator, maintainer
