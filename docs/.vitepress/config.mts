import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "NotionRs",
  description:
    "🦀 Community-driven Notion API client for Rust , offering complete deserialization support and providing a secure way to access properties! 🔒",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Contribute", link: "/contribute" },
    ],

    sidebar: [
      {
        text: "Examples",
        items: [
          { text: "Markdown Examples", link: "/markdown-examples" },
          { text: "Runtime API Examples", link: "/api-examples" },
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
