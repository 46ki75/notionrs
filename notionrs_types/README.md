# notionrs_types — Just a Schema Definition Crate

Notion API schemas may occasionally introduce new fields.
Updating the main crate (`notionrs`) every time leads to breaking changes due to Rust’s strict type system.
To mitigate this, all schema definitions are extracted into this separate crate.

This crate contains only the data structures shared across the `notionrs` ecosystem.

👉 For the API client, see [notionrs](https://docs.rs/notionrs)
