resource "github_repository_ruleset" "branch_restrict_deletion" {
  name        = "branch-restrict-deletion"
  repository  = github_repository.notionrs.name
  target      = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["~DEFAULT_BRANCH", "refs/heads/develop"]
      exclude = []
    }
  }

  rules {
    deletion = true
  }
}

data "github_app" "github_actions" {
  slug = "github-actions"
}

resource "github_repository_ruleset" "branch_require_pr" {
  name        = "branch-require-pr"
  repository  = github_repository.notionrs.name
  target      = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["~DEFAULT_BRANCH", "refs/heads/develop", "refs/heads/release/**/*"]
      exclude = []
    }
  }

  rules {
    required_status_checks {
      required_check {
        context        = "Unit Test (notionrs)"
        integration_id = data.github_app.github_actions.id
      }
      required_check {
        context        = "Unit Test (notionrs-macro)"
        integration_id = data.github_app.github_actions.id
      }
    }
  }
}

resource "github_repository_ruleset" "branch_restrict_mutation_release" {
  name        = "branch-restrict-mutation-release"
  repository  = github_repository.notionrs.name
  target      = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["refs/heads/release/**/*"]
      exclude = []
    }
  }

  rules {
    creation = true
    deletion = true
  }

  bypass_actors {
    actor_id    = 5 # Admin
    actor_type  = "RepositoryRole"
    bypass_mode = "pull_request"
  }
}

resource "github_repository_ruleset" "tag_release_restrict_mutation" {
  name        = "tag-release-restrict-mutation"
  repository  = github_repository.notionrs.name
  target      = "tag"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["refs/tags/v*"]
      exclude = []
    }
  }

  rules {
    creation         = true
    update           = true
    deletion         = true
    non_fast_forward = true
  }

  bypass_actors {
    actor_id    = 5 # Admin
    actor_type  = "RepositoryRole"
    bypass_mode = "always"
  }
}
