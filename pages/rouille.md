---
title: Rouille a web micro-framework in Rust
timestamp: 2024-01-01T14:30:01
author: szabgab
published: true
description: A tutorial on using Rouille to create web application in the Rust programming language.
tags:
    - web
todo:
    - TODO
---

[Rouille](https://crates.io/crates/rouille) is a micro-framework to create web applications in Rust. In this series of articles you'll see
how to get started with it, how to implement various common tasks, and how to deploy it.

* [Hello World with Rouille in text/plain and empty 404 page](/rouille-hello-world-text) `GET`, `text/plain`, `start_server`, `router!`, `text`, `404`, `empty_404`
* [Hello World with Rouille in text/html and home made 404 HTML page](/rouille-hello-world-html) `GET`, `text/html`, `start_server`, `router!`, `html`, `404`, `with_status_code`
* [Handle HTML form - echo with POST](/rouille-echo-post)

* Unfortunately could not find and easy way to test these applications. I opened this [issue](https://github.com/tomaka/rouille/issues/282).

