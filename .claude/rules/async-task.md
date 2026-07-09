---
paths:
  - "notionrs_types/src/object/async_task.rs"
  - "notionrs/src/client/async_task/**"
  - "**/*allow_async*"
---

# Async Task Responses

- `AsyncTaskResponse::Succeeded`/`Failed`'s own `id`/`object` fields describe
  the task envelope itself (`object` is always `"async_task"`, `id` is the
  task's own ID) — not the underlying operation's result. The real fields
  (e.g. a created page's `id`/`markdown`) are nested inside `result`, a raw
  `HashMap<String, serde_json::Value>`. Reading `succeeded.id`/`succeeded.object`
  when you want the operation's result is an easy, silent mistake — it only
  surfaced in this project by actually running an `allow_async(true)` request
  against the live API and observing Notion genuinely go async.
