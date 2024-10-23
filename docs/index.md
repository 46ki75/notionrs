---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "NotionRs"
  text: "Rust Notion API client"
  tagline: A simple Notion API crate that combines response deserialization, request serialization, and secure access to properties.
  actions:
    - theme: brand
      text: Getting Started
      link: /introduction/getting-started
    - theme: alt
      text: What is NotionRs?
      link: /introduction/what-is-notionrs

features:
  - title: Easy to use
    details: You can get started quickly with just your Notion secret token.
    icon: <img src="https://dev.notion.so/front-static/favicon.ico" alt="notion icon" />

  - title: Powered by Rust
    details: Written in Rust, this crate takes full advantage of Rust’s memory safety, concurrency, and strong type system.
    icon: <img src="https://www.rust-lang.org/static/images/favicon.svg" alt="rust icon" />

  - title: Type-safe
    details: The crate supports deserialization of API responses, ensuring type safety.
    icon: 🔒
---
