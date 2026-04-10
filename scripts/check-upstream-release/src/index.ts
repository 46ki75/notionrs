import { getOctokit } from "@actions/github";

const octokit = getOctokit(process.env.GITHUB_TOKEN!);

const title = (version: string) =>
  `[notion-sdk-js] New release detected: ${version}`;

const body = (version: string) =>
  `
A new version of [\`notion-sdk-js\`](https://github.com/makenotion/notion-sdk-js) has been released: **${version}**

Please review the changelog and update the schema as necessary.

- 📄 Release notes: https://github.com/makenotion/notion-sdk-js/releases/tag/${version}

## What should be done?

1. Review the release notes and identify any breaking changes or new features that may require updates to the schema.
2. Check the diff of the release to see if there are any changes that may affect the schema.
3. Implement any necessary updates to the schema to ensure compatibility with the new version of \`notion-sdk-js\`.

---

This issue was generated automatically.
`.trim();

const releases = await octokit.rest.repos.listReleases({
  owner: "makenotion",
  repo: "notion-sdk-js",
});

const { name: version, published_at } = releases.data[0];

if (version == null) {
  throw new Error("name is null.");
}

if (published_at == null) {
  throw new Error("published_at is null.");
}

console.log({ name: version, published_at });

const issues = await octokit.paginate(octokit.rest.issues.listForRepo, {
  owner: "46ki75",
  repo: "notionrs",
  state: "all",
  labels: "notion-sdk-js",
});

console.log(issues.map((issue) => issue.title));

const isAlreadyCreated = issues.some((issue) => issue.title.includes(version));

if (!isAlreadyCreated) {
  await octokit.rest.issues.create({
    owner: "46ki75",
    repo: "notionrs",
    title: title(version),
    body: body(version),
    assignee: "46ki75",
    labels: ["auto-generated", "notion-sdk-js"],
  });
}
