import 'dotenv/config'
import { Client } from '@notionhq/client'
import { writeFileSync } from 'fs'

const SECRET = process.env.NOTION_TOKEN
const PAGE_ID = process.env.NOTION_PAGE_ID

if (SECRET === undefined)
  throw new Error(
    'Notion token is not defined. Please define NOTION_TOKEN in your .env file'
  )

if (PAGE_ID === undefined)
  throw new Error(
    'Notion page id is not defined. Please define NOTION_PAGE_ID in your .env file'
  )

const client = new Client({ auth: SECRET })

const [user] = (await client.users.list({})).results

const sandbox = await client.pages.create({
  parent: { page_id: PAGE_ID },
  properties: {
    title: {
      type: 'title',
      title: [
        {
          type: 'text',
          text: {
            content: `[NotionRs]`
          },
          annotations: { color: 'blue' }
        },
        {
          type: 'text',
          text: {
            content: ` Integration Test Sandbox `
          }
        },
        {
          type: 'text',
          text: {
            content: `(${new Date().toISOString()})`
          }
        }
      ]
    }
  }
})

const { id: NOTION_IT_SANDBOX_ID } = sandbox

// # --------------------------------------------------------------------------------
//
// block
//
// # --------------------------------------------------------------------------------

const blockPage = await client.pages.create({
  parent: { page_id: sandbox.id },
  properties: {
    title: {
      type: 'title',
      title: [{ type: 'text', text: { content: 'CRUD Block' } }]
    }
  }
})

const { id: NOTION_IT_CRUD_BLOCK_PAGE_ID } = blockPage

// # --------------------------------------------------------------------------------
//
// Database
//
// # --------------------------------------------------------------------------------

const relatedDatabase = await client.databases.create({
  parent: { page_id: sandbox.id },
  title: [{ type: 'text', text: { content: 'delete_database' } }],
  properties: {
    Title: { title: {} }
  }
})

const database = await client.databases.create({
  parent: { page_id: sandbox.id },
  title: [{ type: 'text', text: { content: 'Query, Retrieve database' } }],
  properties: {
    Title: { title: {} },
    Checkbox: { checkbox: {} },
    Date: { date: {} },
    'Files & media': { files: {} },
    Number: { number: { format: 'number' } },
    User: { people: {} },
    'Phone Number': { phone_number: {} },
    formula: { formula: { expression: 'prop("Number") / 2' } },
    Text: { rich_text: {} },
    Select: { select: { options: [{ name: 'select-1' }] } },
    'Multi-select': {
      multi_select: { options: [{ name: 'select-1' }, { name: 'select-2' }] }
    },
    URL: { url: {} },
    email: { email: {} },
    Relation: {
      relation: { database_id: relatedDatabase.id, single_property: {} }
    },
    'Created time': { created_time: {} },
    CreatedBy: { created_by: {} },
    LastUpdatedAt: { last_edited_time: {} },
    LastUpdatedBy: { last_edited_by: {} },
    ID: { unique_id: {} }
  }
})

const { id: NOTION_IT_DATABASE_ID } = database

await client.databases.update({
  database_id: database.id,
  properties: {
    Rollup: {
      rollup: {
        relation_property_name: 'Relation',
        relation_property_id: database.properties.Relation.id,
        rollup_property_name: 'Title',
        function: 'count'
      }
    }
  }
})

// # --------------------------------------------------------------------------------
//
// Page
//
// # --------------------------------------------------------------------------------

const { id: NOTION_IT_CRUD_PAGE_ID } = await client.pages.create({
  parent: { database_id: database.id },
  properties: {
    Title: { title: [{ type: 'text', text: { content: 'Title 1' } }] },
    Checkbox: { checkbox: true },
    Date: { date: { start: '2021-01-01' } },
    Number: { number: 1 },
    'Phone Number': { phone_number: '+1234567890' },
    URL: { url: 'https://www.notion.so' },
    email: { email: 'info@example.com' },
    // formula: { formula: { type: 'string', string: 'Hello' } },
    Text: { rich_text: [{ type: 'text', text: { content: 'Hello' } }] },
    Select: { select: { name: 'select-1' } },
    'Multi-select': { multi_select: [{ name: 'select-1' }] },
    Relation: { relation: [{ id: blockPage.id }] },
    'Files & media': {
      files: [
        { external: { url: 'https://example.com/file.txt' }, name: 'File Name' }
      ]
    },
    User: { people: [] }
  }
})

// save

const data = `NOTION_IT_CRUD_BLOCK_PAGE_ID="${NOTION_IT_CRUD_BLOCK_PAGE_ID}"
NOTION_IT_DATABASE_ID="${NOTION_IT_DATABASE_ID}"
NOTION_IT_CRUD_PAGE_ID="${NOTION_IT_CRUD_PAGE_ID}"
NOTION_IT_SANDBOX_ID="${NOTION_IT_SANDBOX_ID}"
`

writeFileSync('notionrs/.env.test', data, 'utf-8')
