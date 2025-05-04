import { getOctokit } from "@actions/github";

const octokit = getOctokit(process.env.GITHUB_TOKEN!);

await octokit.rest.issues.create({
  owner: "46ki75",
  repo: "notionrs",
  title: "Issue Title",
  body: "Issue Body",
  assignee: "46ki75",
  labels: [],
});
