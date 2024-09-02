import Link from "next/link";
import React, { useState } from "react";
import dayjs from "dayjs";
import {
  EmptyFireSvg,
  FireSvg,
} from "./Svgs";
import { Calendar } from "./Calendar";
import { useBoundStore } from "~/hooks/useBoundStore";
import { Flag } from "./Flag";
import type { LoginScreenState } from "./LoginScreen";
import { LoginScreen } from "./LoginScreen";
import { currentUser } from "~/utils/userData";

/* 
  TODO foookin courses babeyyy
*/

export const RightBar = () => {
  const loggedIn = () => { return currentUser.loggedIn };
  const streak = useBoundStore((x) => x.streak);
  const language = useBoundStore((x) => x.language);

  const [languagesShown, setLanguagesShown] = useState(false);

  const [streakShown, setStreakShown] = useState(false);
  const [now, setNow] = useState(dayjs());

  const [loginScreenState, setLoginScreenState] =
    useState<LoginScreenState>("HIDDEN");

  return (
    <>
      <aside className="text-white px-5 py-4 bg-dark-purple rounded-2xl sticky top-10 hidden w-96 flex-col gap-6 self-start sm:flex">
        <article className="my-6 flex justify-between gap-4">
          <div
            className="relative flex cursor-default items-center gap-2 rounded-xl p-3 font-bold text-white hover:bg-pink-ish"
            onMouseEnter={() => setLanguagesShown(true)}
            onMouseLeave={() => setLanguagesShown(false)}
            onClick={() => setLanguagesShown((x) => !x)}
            role="button"
            tabIndex={0}
          >
            <Flag language={language} width={45} />
            <div>{language.name}</div>
            <div
              className="absolute top-full z-10 rounded-2xl border-2 border-white bg-dark-purple"
              style={{
                left: "calc(50% - 150px)",
                width: 300,
                display: languagesShown ? "block" : "none",
              }}
            >
              <h2 className="px-5 py-3 font-bold text-white">
                my courses
              </h2>
              <button className="flex w-full items-center gap-3 bg-dark-purple px-5 py-3 text-left font-bold hover:bg-pink-ish">
                <Flag language={language} width={38} />
                <span className="text-white">{language.name}</span>
              </button>
              <Link
                className="flex w-full items-center gap-3 rounded-b-2xl px-5 py-3 text-left font-bold hover:bg-darker-purple"
                href="/register"
              >
                <span className="flex items-center justify-center rounded-lg px-2 text-lg font-bold text-white">
                  {'>'}
                </span>
                <span className="text-white">change course</span>
              </Link>
            </div>
          </div>
          <span
            className="relative flex items-center gap-2 rounded-xl p-3 font-bold text-white hover:bg-white"
            onMouseEnter={() => setStreakShown(true)}
            onMouseLeave={() => {
              setStreakShown(false);
              setNow(dayjs());
            }}
            onClick={(event) => {
              if (event.target !== event.currentTarget) return;
              setStreakShown((x) => !x);
              setNow(dayjs());
            }}
            role="button"
            tabIndex={0}
          >
            <div className="pointer-events-none">
              {streak > 0 ? <FireSvg /> : <EmptyFireSvg />}
            </div>
            <span className={streak > 0 ? "text-pink-ish" : "text-white"}>
              {streak}
            </span>
            <div
              className="absolute top-full z-10 flex flex-col gap-5 rounded-2xl border-2 border-white bg-white p-5 text-black"
              style={{
                left: "calc(50% - 200px)",
                width: 400,
                display: streakShown ? "flex" : "none",
              }}
            >
              <h2 className="text-center text-lg font-bold">streak</h2>
              <p className="text-center text-sm font-normal text-black">
                {`do not forget to come here every day, for a few minutes at least - or else the streak will be gone...`}
              </p>
              <Calendar now={now} setNow={setNow} />
            </div>
          </span>
        </article>
        {!loggedIn() && (
          <CreateAProfileSection setLoginScreenState={setLoginScreenState} />
        )}
      </aside>
      <LoginScreen
        loginScreenState={loginScreenState}
        setLoginScreenState={setLoginScreenState}
      />
    </>
  );
};

const CreateAProfileSection = ({
  setLoginScreenState,
}: {
  setLoginScreenState: React.Dispatch<React.SetStateAction<LoginScreenState>>;
}) => {
  return (
    <article className="bg-darker-purple flex flex-col gap-5 rounded-2xl border-2 border-white p-6 font-bold">
      <h2 className="text-xl">not yet signed up? do so NOW, you lil bitch</h2>
      <button
        className="rounded-2xl border-b-4 border-dark-purple bg-dark-purple py-3 text-white transition hover:border-pink-ish hover:bg-pink-ish"
        onClick={() => setLoginScreenState("SIGNUP")}
      >
        make a profile
      </button>
      <button
        className="rounded-2xl border-b-4 border-dark-purple bg-dark-purple py-3 text-white transition hover:border-pink-ish hover:bg-pink-ish"
        onClick={() => setLoginScreenState("LOGIN")}
      >
        sign in
      </button>
    </article>
  );
};
