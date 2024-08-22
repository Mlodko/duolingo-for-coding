import { type NextPage } from "next";
import React from "react";
import { LanguageHeader } from "~/components/LanguageHeader";
import { useLoginScreen, LoginScreen } from "~/components/LoginScreen";
import _bgSnow from "../../public/bg-snow.svg";
import type { StaticImageData } from "next/image";

const bgSnow = _bgSnow as StaticImageData;

const Home: NextPage = () => {
  const { loginScreenState, setLoginScreenState } = useLoginScreen();
  return (
    <main
      className="flex min-h-screen flex-col items-center justify-center bg-darker-purple text-white"
      style={{ backgroundImage: `url(${bgSnow.src})` }}
    >
      <LanguageHeader />
      <div className="flex w-full flex-col items-center justify-center gap-3 px-4 py-16 md:flex-row md:gap-36">
        <div>
          <p className="mb-6 max-w-[600px] text-center text-3xl font-bold md:mb-12">
            learn the programming language of<br></br>your choice, for free, no credit card, no bullshit.<br></br><br></br>are you ready?
          </p>
          <div className="mx-auto mt-4 flex w-fit flex-col items-center gap-3">
            <button
              onClick={() => setLoginScreenState("SIGNUP")}
              className="w-full rounded-2xl border-2 border-b-4 border-pink-ish bg-dark-purple-600 px-10 py-3 text-center text-white font-bold transition hover:bg-pink-ish md:min-w-[320px]"
            >
              sign me up
            </button>
            <button
              className="w-full rounded-2xl border-2 border-b-4 border-pink-ish bg-darker-purple px-8 py-3 font-bold text-white transition hover:bg-pink-ish md:min-w-[320px]"
              onClick={() => setLoginScreenState("LOGIN")}
            >
              let me sign in
            </button>
          </div>
        </div>
      </div>
      <LoginScreen
        loginScreenState={loginScreenState}
        setLoginScreenState={setLoginScreenState}
      />
    </main>
  );
};

export default Home;
