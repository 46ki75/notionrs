import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "NotionRs",
  description:
    "🦀 Community-driven Notion API client for Rust , offering complete deserialization support and providing a secure way to access properties! 🔒",
  base: "/notionrs/",
  markdown: {
    theme: {
      light: "vitesse-light",
      dark: "vitesse-dark",
    }
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config

    search: { provider: "local" },

    nav: [
      { text: "Giude", link: "/introduction/getting-started" },
      { text: "Contribute", link: "/contribute" },
    ],

    sidebar: [
      {
        text: "Introduction",
        items: [
          { text: "What is notionrs?", link: "/introduction/what-is-notionrs" },
          { text: "Getting Started", link: "/introduction/getting-started" },
        ],
      },
      {
        text: "Guide - Block",
        items: [
          { text: "Append Block Children", link: "/block/append-block-children" },
          { text: "Get Block", link: "/block/get-block" },
          { text: "Get Block Children", link: "/block/get-block-children" },
          { text: "Update Block", link: "/block/update-block" },
          { text: "Delete Block", link: "/block/delete-block" },
        ],
      },
      {
        text: "Guide - Page",
        items: [
          { text: "Create Page", link: "/page/create-page" },
          { text: "Get Page", link: "/page/get-page" },
          { text: "Get Page Property Item", link: "/page/get-page-property-item" },
          { text: "Update Page", link: "/page/update-page" },
        ],
      },
      {
        text: "Guide - Database",
        items: [
          { text: "Create Database", link: "/database/create-database" },
          { text: "Query Database", link: "/database/query-database" },
          { text: "Retrieve Database", link: "/database/retrieve-database" },
          { text: "Update Database", link: "/database/update-database" },
        ],
      },
      {
        text: "Guide - User",
        items: [
          { text: "List Users", link: "/user/list-users" },
          { text: "Get User", link: "/user/get-user" },
          { text: "Get Self", link: "/user/get-self" },
        ],
      },
      {
        text: "Contribute",
        items: [
          {
            text: "For Crate Developers",
            link: "/contribute/",
          },
          { text: "Unit Tests", link: "/contribute/unit-test" },
          { text: "Integration Tests", link: "/contribute/integration-test" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/46ki75/notionrs" },
    ],
  },
});
