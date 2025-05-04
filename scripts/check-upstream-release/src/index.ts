import { getOctokit } from "@actions/github";

const octokit = getOctokit(process.env.GITHUB_TOKEN!);

const body = (version: string) =>
  `
A new version of [\`notion-sdk-js\`](https://github.com/makenotion/notion-sdk-js) has been released: **${version}**

Please review the changelog and update the schema as necessary.

- 📄 Release notes: https://github.com/makenotion/notion-sdk-js/releases/tag/${version}
`.trim();

const releases = await octokit.rest.repos.listReleases({
  owner: "makenotion",
  repo: "notion-sdk-js",
});

const { name, published_at } = releases.data[0];

if (name == null) {
  throw new Error("name is null.");
}

if (published_at == null) {
  throw new Error("published_at is null.");
}

if (
  // TODO:
  //   new Date(published_at).getTime() >
  //   new Date(Date.now() + 24 * 60 * 60 * 1000).getTime()
  true
) {
  console.log({ name, published_at });

  await octokit.rest.issues.create({
    owner: "46ki75",
    repo: "notionrs",
    title: `[notion-sdk-js] New release detected: ${name}`,
    body: body(name),
    assignee: "46ki75",
    labels: [],
  });
}
