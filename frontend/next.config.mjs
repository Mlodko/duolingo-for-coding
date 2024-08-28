await import("./src/env.mjs");

/** @type {import("next").NextConfig} */
const config = {
  reactStrictMode: false,

  i18n: {
    locales: ["en"],
    defaultLocale: "en",
  },
};
export default config;
