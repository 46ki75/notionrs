import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "NotionRs",
  description:
    "🦀 Community-driven Notion API client for Rust , offering complete deserialization support and providing a secure way to access properties! 🔒",
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
          { text: "Audio Block", link: "/block/audio" },
          { text: "Delete a block", link: "/block/delete-a-block" },
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
      { icon: "github", link: "https://github.com/vuejs/vitepress" },
    ],
  },
});
