// Common
resource "github_issue_label" "bug" {
  repository  = github_repository.notionrs.name
  name        = "Bug"
  color       = "8e3636"
  description = "Something isn't working."
}

resource "github_issue_label" "duplicate" {
  repository  = github_repository.notionrs.name
  name        = "duplicate"
  color       = "bec2ca"
  description = "This issue or pull request already exists."
}

resource "github_issue_label" "enhancement" {
  repository  = github_repository.notionrs.name
  name        = "enhancement"
  color       = "aebed9"
  description = "New feature or request."
}

resource "github_issue_label" "question" {
  repository  = github_repository.notionrs.name
  name        = "question"
  color       = "e4b4ce"
  description = "Further information is requested."
}

resource "github_issue_label" "good_first_issue" {
  repository  = github_repository.notionrs.name
  name        = "good first issue"
  color       = "cab7dd"
  description = "Good for newcomers"
}

// PRs
resource "github_issue_label" "dependencies" {
  repository  = github_repository.notionrs.name
  name        = "dependencies"
  color       = "7e50ab"
  description = "Pull requests that update a dependency file."
}

resource "github_issue_label" "documentation" {
  repository  = github_repository.notionrs.name
  name        = "documentation"
  color       = "449763"
  description = "Improvements or additions to documentation."
}

resource "github_issue_label" "refactor" {
  repository  = github_repository.notionrs.name
  name        = "refactor"
  color       = "7e50ab"
  description = "Improve code without changing behavior."
}

// Languages
resource "github_issue_label" "npm" {
  repository  = github_repository.notionrs.name
  name        = "npm"
  color       = "9e2219"
  description = "Pull requests that update package.json or its lock file."
}

resource "github_issue_label" "terraform" {
  repository  = github_repository.notionrs.name
  name        = "Terraform"
  color       = "7c53b4"
  description = "Pull requests that update terraform lock file."
}

resource "github_issue_label" "rust" {
  repository  = github_repository.notionrs.name
  name        = "Rust"
  color       = "f6c5a2"
  description = "Pull requests that update Cargo.toml or its lock file."
}

resource "github_issue_label" "release" {
  repository  = github_repository.notionrs.name
  name        = "Release"
  color       = "3c557f"
  description = "Merge release changes into the main branch."
}

resource "github_issue_label" "auto_generated" {
  repository  = github_repository.notionrs.name
  name        = "auto-generated"
  color       = "555b67"
  description = "Issues created automatically by workflows or bots"
}

resource "github_issue_label" "notion_sdk_js" {
  repository  = github_repository.notionrs.name
  name        = "notion-sdk-js"
  color       = "868e9c"
  description = "Related to the upstream Notion SDK for JavaScript"
}
