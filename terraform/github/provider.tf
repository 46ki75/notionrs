terraform {
  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }

  cloud {
    organization = "46ki75"
    hostname     = "app.terraform.io"

    workspaces {
      project = "notionrs"
      name    = "shared"
    }
  }
}
