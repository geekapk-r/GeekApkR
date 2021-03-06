# Deployment

> [Configuring rocket.rs](https://rocket.rs/guide/configuration/)

1. prepare for GeekServer executable

2. prepare for PostgreSQL and Apache HTTPd

3. create a new user for GeekServer

4. create data dir for GeekServer(geekapk user has write permission)

5. initialize data dir(must have [geekapk.ini](geekapk.example.d/geekapk.example.ini) and [geekapk_images/](geekapk.example.d/geekapk_images/))

6. initialize Postgres user and [database](geekapk.example.d/dbinit.d/) (geekapk_main, geekapk_secure, geekapk_comments, geekapk_pops)

7. create a systemd service for GeekServer, start it.

8. proxy the HTTP API using [proxy htaccess](proxy_htaccess) (if you are using reverse proxies like CloudFlare, try proxy_apache/proxy_nginx)
