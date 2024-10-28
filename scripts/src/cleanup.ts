import dotenv from 'dotenv'
dotenv.config({ path: '.env' })
dotenv.config({ path: '.env.test' })

import { Client } from '@notionhq/client'

const SECRET = process.env.NOTION_TOKEN

if (SECRET === undefined)
  throw new Error(
    'Notion token is not defined. Please define NOTION_TOKEN in your .env file'
  )

const client = new Client({ auth: SECRET })

if (process.env.NOTION_IT_ROOT_ID != null)
  await client.pages.update({
    page_id: process.env.NOTION_IT_ROOT_ID,
    archived: true,
    in_trash: true
  })
