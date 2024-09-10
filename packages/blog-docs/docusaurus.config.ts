import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

const config: Config = {
  title: "Palform",
  tagline: "Build beautiful end-to-end encrypted forms for free",
  favicon: "/img/logo.svg",

  // Set the production url of your site here
  url: "https://docs.palform.app",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  scripts: [
    {
      src: "https://plausible.io/js/script.js",
      defer: true,
      "data-domain": "palform.app",
    },
  ],

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          routeBasePath: "/",
        },
        blog: {
          showReadingTime: true,
          feedOptions: {
            type: ["rss", "atom"],
            xslt: true,
          },
        },
        sitemap: {
          lastmod: "datetime",
          changefreq: "daily",
          filename: "sitemap.xml",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    navbar: {
      title: "Palform",
      logo: {
        alt: "Palform Logo",
        src: "/img/logo.svg",
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "docsSidebar",
          position: "left",
          label: "Support",
        },
        { to: "/blog", label: "Blog", position: "left" },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          items: [
            {
              label: "Palform",
              to: "https://palform.app/",
            },
            {
              label: "Log in",
              to: "https://dash.palform.app/auth/login",
            },
          ],
        },
        {
          title: "Legal",
          items: [
            {
              label: "Terms of Use",
              href: "https://palform.app/legal/terms",
            },
            {
              label: "Privacy Policy",
              href: "https://palform.app/legal/privacy",
            },
          ],
        },
        {
          title: "Support",
          items: [
            {
              label: "Contact us",
              to: "/",
            },
            {
              label: "Our roadmap",
              href: "https://palform.app/support/roadmap",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Palform Ltd. Registered company 15796859.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
