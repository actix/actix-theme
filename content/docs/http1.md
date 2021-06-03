---
title: HTTP/1
menu: docs_protocols
weight: 280
---

For very specific purposes, you may need a server that does not upgrade to H2
even if the client desires to. In that case you might need to use the H1 service.

# H1 Service

In general, browsers will not use HTTP/2 without HTTPS, that is why the example uses HTTPS.

```toml
[dependencies]
actix-cors = "0.5"
actix-service = "1"
actix-http = { version = "2", features = ["openssl"] }
actix-web = { version = "3", features = ["openssl"] }
openssl = { version = "0.10", features = ["v110"] }
```
{{< include-example example="http1" file="main.rs" section="main" >}}
