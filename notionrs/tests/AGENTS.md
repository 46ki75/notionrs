# Integration Tests

- Page: "`46ki75/notionrs` Integration Test Page Root"
  - Page: "Read-only: Integration Test"
  - Page: "Mutable: Integration Test"

## Readonly

- Root page name: "Read-only: Integration Test"
- Notion API key name: `integration-test-readonly`
- `.env` file path: `notionrs/.env.readonly`
- Capabilities
  - Content capabilities
    - [x] Read content
    - [ ] Update content
    - [ ] Insert content
  - Comment capabilities
    - [x] Read comments
    - [ ] Insert comments
  - User capabilities
    - [ ] No user information
    - [x] Read user information without email addresses
    - [ ] Read user information including email addresses

## Mutable

- Root page name: ""
- Notion API key name: `integration-test-mutable`
- `.env` file path: `notionrs/.env.mutable`
- Capabilities
  - Content capabilities
    - [x] Read content
    - [x] Update content
    - [x] Insert content
  - Comment capabilities
    - [x] Read comments
    - [x] Insert comments
  - User capabilities
    - [ ] No user information
    - [ ] Read user information without email addresses
    - [x] Read user information including email addresses
