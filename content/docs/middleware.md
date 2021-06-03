---
title: Middleware
menu: docs_advanced
weight: 220
---

# Middleware

Actix-web's middleware system allows us to add additional behavior to request/response
processing.  Middleware can hook into an incoming request process, enabling us to modify
requests as well as halt request processing to return a response early.

Middleware can also hook into response processing.

Typically, middleware is involved in the following actions:

* Pre-process the Request
* Post-process a Response
* Modify application state
* Access external services (redis, logging, sessions)

Middleware is registered for each `App`, `scope`, or `Resource` and executed in opposite
order as registration.  In general, a *middleware* is a type that implements the
[*Service trait*][servicetrait] and [*Transform trait*][transformtrait].  Each method in
the traits has a default implementation. Each method can return a result immediately
or a *future* object.

The following demonstrates creating a simple middleware:

{{< include-example example="middleware" file="main.rs" section="simple" >}}

Alternatively, for simple use cases, you can use [*wrap_fn*][wrap_fn] to create small, ad-hoc middleware:

{{< include-example example="middleware" file="wrap_fn.rs" section="wrap-fn" >}}

> Actix-web provides several useful middleware, such as *logging*, *user sessions*,
> *compress*, etc.

# Logging

Logging is implemented as a middleware.  It is common to register a logging middleware
as the first middleware for the application.  Logging middleware must be registered for
each application.

The `Logger` middleware uses the standard log crate to log information. You should enable logger
for *actix_web* package to see access log ([env_logger][envlogger] or similar).

## Usage

Create `Logger` middleware with the specified `format`.  Default `Logger` can be created
with `default` method, it uses the default format:

```ignore
  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
```

{{< include-example example="middleware" file="logger.rs" section="logger" >}}

The following is an example of the default logging format:

```
INFO:actix_web::middleware::logger: 127.0.0.1:59934 [02/Dec/2017:00:21:43 -0800] "GET / HTTP/1.1" 302 0 "-" "curl/7.54.0" 0.000397
INFO:actix_web::middleware::logger: 127.0.0.1:59947 [02/Dec/2017:00:22:40 -0800] "GET /index.html HTTP/1.1" 200 0 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:57.0) Gecko/20100101 Firefox/57.0" 0.000646
```

## Format

- `%%`  The percent sign
- `%a`  Remote IP-address (IP-address of proxy if using reverse proxy)
- `%t`  Time when the request was started to process
- `%P`  The process ID of the child that serviced the request
- `%r`  First line of request
- `%s`  Response status code
- `%b`  Size of response in bytes, including HTTP headers
- `%T`  Time taken to serve the request, in seconds with floating fraction in .06f format
- `%D`  Time taken to serve the request, in milliseconds
- `%{FOO}i`  request.headers['FOO']
- `%{FOO}o`  response.headers['FOO']
- `%{FOO}e`  os.environ['FOO']

## Default headers

To set default response headers, the `DefaultHeaders` middleware can be used. The
*DefaultHeaders* middleware does not set the header if response headers already contain
a specified header.

{{< include-example example="middleware" file="default_headers.rs" section="default-headers" >}}

## User sessions

Actix-web provides a general solution for session management. The
[**actix-session**][actixsession] middleware can use multiple backend types to store session data.

> By default, only cookie session backend is implemented. Other backend implementations
> can be added.

[**CookieSession**][cookiesession] uses cookies as session storage. `CookieSessionBackend`
creates sessions which are limited to storing fewer than 4000 bytes of data, as the payload
must fit into a single cookie. An internal server error is generated if a session
contains more than 4000 bytes.

A cookie may have a security policy of *signed* or *private*. Each has a respective
`CookieSession` constructor.

A *signed* cookie may be viewed but not modified by the client. A *private* cookie may
neither be viewed nor modified by the client.

The constructors take a key as an argument. This is the private key for cookie session -
when this value is changed, all session data is lost.

In general, you create a `SessionStorage` middleware and initialize it with specific
backend implementation, such as a `CookieSession`. To access session data the
[`Session`][requestsession] extractor must be used. This method returns a
[*Session*][sessionobj] object, which allows us to get or set session data.

{{< include-example example="middleware" file="user_sessions.rs" section="user-session" >}}

# Error handlers

`ErrorHandlers` middleware allows us to provide custom handlers for responses.

You can use the `ErrorHandlers::handler()` method to register a custom error handler
for a specific status code. You can modify an existing response or create a completly new
one. The error handler can return a response immediately or return a future that resolves
into a response.

{{< include-example example="middleware" file="errorhandler.rs" section="error-handler" >}}

# Authorization

[actix-casbin-auth](https://github.com/casbin-rs/actix-casbin-auth) middleware provides
authorization based on access control models like ACL, RBAC, ABAC. Casbin doesn't
provide authentication (who you are), so you need to implement an Authentication Middleware
to identify a user.

{{< include-example example="middleware" file="authorization.rs" section="authorization" >}}

[sessionobj]: https://docs.rs/actix-session/0.3.0/actix_session/struct.Session.html
[requestsession]: https://docs.rs/actix-session/0.3.0/actix_session/struct.Session.html
[cookiesession]: https://docs.rs/actix-session/0.3.0/actix_session/struct.CookieSession.html
[actixsession]: https://docs.rs/actix-session/0.3.0/actix_session/
[envlogger]: https://docs.rs/env_logger/*/env_logger/
[servicetrait]: https://docs.rs/actix-web/3/actix_web/dev/trait.Service.html
[transformtrait]: https://docs.rs/actix-web/3/actix_web/dev/trait.Transform.html
[wrap_fn]: https://docs.rs/actix-web/3/actix_web/struct.App.html#method.wrap_fn
