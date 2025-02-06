# Rust-Axum-Askama App

This is an example web application to get you up and runnig with web development in Rust.

This template uses the following technologies:

- [Axum](https://github.com/tokio-rs/axum) as a web framework
- [Askama](https://github.com/rinja-rs/askama) as the templating engine
- [Htmx](https://htmx.org) for creating the pages
- [Tailwindcss](https://tailwindcss.com) for styling
- [Alpine.js](https://alpinejs.dev) to add more interactivity

# Running and Developing

For the template to work you need the following dependencies:
- Rust toolchain
- Tailwindcss cli
- Just

To get this template running just clone the repository and run the following Just command:

```sh
just run
```

This will build the app and run it. Go to [localhost:3000](http://127.0.0.1:3000) to see the page running.

If you want to start the app in developing mode so the code rebuilds on changes run this command:

```sh
just dev
```
