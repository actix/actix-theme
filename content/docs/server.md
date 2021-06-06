---
title: Server
menu: docs_basics
weight: 150
---

# The HTTP Server

The [**HttpServer**][httpserverstruct] type is responsible for serving HTTP requests.

`HttpServer` accepts an application factory as a parameter, and the application factory
must have `Send` + `Sync` boundaries. More about that in the *multi-threading* section.

To bind to a specific socket address, [`bind()`][bindmethod] must be used, and it may be
called multiple times. To bind ssl socket, [`bind_openssl()`][bindopensslmethod] or
[`bind_rustls()`][bindrusttls] should be used. To run the HTTP server, use the `HttpServer::run()`
method.

{{< include-example example="server" section="main" >}}

The `run()` method returns an instance of the [`Server`][server] type. Methods of server type
could be used for managing the HTTP server

- `pause()` - Pause accepting incoming connections
- `resume()` - Resume accepting incoming connections
- `stop()` - Stop incoming connection processing, stop all workers and exit

The following example shows how to start the HTTP server in a separate thread.

{{< include-example example="server" file="signals.rs" section="signals" >}}

## Multi-threading

`HttpServer` automatically starts a number of HTTP *workers*, by default this number is
equal to the number of logical CPUs in the system. This number can be overridden with the
[`HttpServer::workers()`][workers] method.

{{< include-example example="server" file="workers.rs" section="workers" >}}

Once the workers are created, they each receive a separate *application* instance to handle
requests. Application state is not shared between the threads, and handlers are free to manipulate
their copy of the state with no concurrency concerns.

> Application state does not need to be `Send` or `Sync`, but application
factories must be `Send` + `Sync`.

To share state between worker threads, use an `Arc`. Special care should be taken once sharing and
synchronization are introduced. In many cases, performance costs are inadvertently introduced as a
result of locking the shared state for modifications.

In some cases these costs can be alleviated using more efficient locking strategies, for example
using [read/write locks](https://doc.rust-lang.org/std/sync/struct.RwLock.html) instead of
[mutexes](https://doc.rust-lang.org/std/sync/struct.Mutex.html) to achieve non-exclusive locking,
but the most performant implementations often tend to be ones in which no locking occurs at all.

Since each worker thread processes its requests sequentially, handlers which block the current
thread will cause the current worker to stop processing new requests:

```rust
fn my_handler() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to hang!
    "response"
}
```

For this reason, any long, non-cpu-bound operation (e.g. I/O, database operations, etc.) should be
expressed as futures or asynchronous functions. Async handlers get executed concurrently by worker
threads and thus don't block execution:

```rust
async fn my_handler() -> impl Responder {
    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}
```

The same limitation applies to extractors as well. When a handler function receives an argument
which implements `FromRequest`, and that implementation blocks the current thread, the worker thread
will block when running the handler. Special attention must be given when implementing extractors
for this very reason, and they should also be implemented asynchronously where needed.

## SSL

There are two features for the ssl server: `rustls` and `openssl`. The `rustls` feature is for
`rustls` integration and `openssl` is for `openssl`.

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["openssl"] }
openssl = { version = "0.10" }
```

{{< include-example example="server" file="ssl.rs" section="ssl" >}}

> **Note**: the *HTTP/2.0* protocol requires [tls alpn][tlsalpn].
> At the moment, only `openssl` has `alpn` support.
> For a full example, check out [examples/openssl][exampleopenssl].

To create the key.pem and cert.pem use the command. **Fill in your own subject**
```bash
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```
To remove the password, then copy nopass.pem to key.pem
```bash
$ openssl rsa -in key.pem -out nopass.pem
```

## Keep-Alive

Actix can wait for requests on a keep-alive connection.

> *keep alive* connection behavior is defined by server settings.

- `75`, `Some(75)`, `KeepAlive::Timeout(75)` - enable 75 second *keep alive* timer.
- `None` or `KeepAlive::Disabled` - disable *keep alive*.
- `KeepAlive::Tcp(75)` - use `SO_KEEPALIVE` socket option.

{{< include-example example="server" file="keep_alive.rs" section="keep-alive" >}}

If the first option above is selected, then *keep alive* state is calculated
based on the response's [*connection-type*][httpconnectiontype]. By default no
explicit connection type is defined for an HTTP response; in that case *keep alive*
is determined based on the request's HTTP version:

> *keep alive* is **off** for *HTTP/1.0* and is **on** for *HTTP/1.1* and *HTTP/2.0*.

The *connection type* can be changed with the following [`HttpResponseBuilder`][httpresponsebuilder] methods:

- [`keep_alive()`][hrb-method-keep_alive] - Set connection type to KeepAlive
- [`upgrade()`][hrb-method-upgrade] - Set connection type to Upgrade
- [`force_close()`][hrb-method-force_close] - Force close connection, even if it is marked as keep-alive

{{< include-example example="server" file="keep_alive_tp.rs" section="example" >}}

## Graceful shutdown

`HttpServer` supports graceful shutdown. After receiving a stop signal, workers
have a specific amount of time to finish serving requests. Any workers still alive after the
timeout are force-dropped. By default the shutdown timeout is set to 30 seconds.  You
can change this parameter with the [`HttpServer::shutdown_timeout()`][shutdowntimeout]
method.

`HttpServer` handles several OS signals. *CTRL-C* is available on all OSs, other signals
are available on unix systems.

- *SIGINT* - Force shutdown workers
- *SIGTERM* - Graceful shutdown workers
- *SIGQUIT* - Force shutdown workers

> It is possible to disable signal handling with
[`HttpServer::disable_signals()`][disablesignals] method.

[server]: https://docs.rs/actix-web/3/actix_web/dev/struct.Server.html
[httpconnectiontype]:     https://docs.rs/actix-web/3/actix_web/http/enum.ConnectionType.html
[httpresponsebuilder]:    https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html
[hrb-method-keep_alive]:  https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html#method.keep_alive
[hrb-method-upgrade]:     https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html#method.upgrade
[hrb-method-force_close]: https://docs.rs/actix-web/3/actix_web/dev/struct.HttpResponseBuilder.html#method.force_close
[httpserverstruct]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html
[bindmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind
[bindopensslmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_openssl
[bindrusttls]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_rustls
[workers]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.workers
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[exampleopenssl]: https://github.com/actix/examples/tree/master/security/openssl
[shutdowntimeout]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.shutdown_timeout
[disablesignals]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.disable_signals
