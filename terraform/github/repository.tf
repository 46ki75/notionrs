resource "github_repository" "notionrs" {
  name         = "notionrs"
  description  = "🦀 Community-driven Notion API client for Rust, offering complete deserialization support and providing a secure way to access properties!"
  homepage_url = "https://crates.io/crates/notionrs"

  has_downloads        = false
  has_issues           = true
  vulnerability_alerts = true

  pages {
    build_type = "workflow"

    source {
      branch = "main"
      path   = "/"
    }
  }
}
