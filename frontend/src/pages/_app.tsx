import { type AppType } from "next/dist/shared/lib/utils";
import Head from "next/head";

import "~/styles/globals.css";

const MyApp: AppType = ({ Component, pageProps }) => {
  return (
    <>
      <Head>
        <title>code samurai</title>
        <meta
          name="description"
          content="code samurai - a Duolingo-inspired web-app for everyone wanting to learn programming languages"
        />
        <link rel="icon" href="/favicon.ico" />
        <meta name="theme-color" content="dark" />
        <link rel="manifest" href="/app.webmanifest" />
      </Head>
      <Component {...pageProps} />
    </>
  );
};

export default MyApp;
