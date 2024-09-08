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
import _lvlUpStar from "../../public/drawing-1.svg";
import { StaticImageData } from "next/image";

const lvlUpStar = _lvlUpStar as StaticImageData;

/* 
  TODO foookin courses babeyyy
*/

export const RightBar = () => {
  const loggedIn = () => { return currentUser.loggedIn };
  const currentLvl = () => { return currentUser.level.level};
  const language = useBoundStore((x) => x.language);

  const [languagesShown, setLanguagesShown] = useState(false);

  const [levelShown, setLevelShown] = useState(false);
  
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
          {/* vvv Here I am gonna add latest XP changes or sth */}
          <span
            className="relative flex items-center gap-2 rounded-xl p-3 font-bold text-white hover:bg-pink-ish hover:text-pink-ish"
            onMouseEnter={() => setLevelShown(true)}
            onMouseLeave={() => {
              setLevelShown(false);
            }}
            onClick={(event) => {
              if (event.target !== event.currentTarget) return;
              setLevelShown((x) => !x);
            }}
            role="button"
            tabIndex={0}
          >
            <div className="pointer-events-none">
              <img src={lvlUpStar.src} height={30} width={30}/>
            </div>
            <span className="text-white">
              {currentLvl()}
            </span>
            <div
              className="absolute top-full z-10 flex flex-col gap-5 rounded-2xl border-2 border-white bg-white p-5 text-black"
              style={{
                left: "calc(50% - 200px)",
                width: 400,
                display: levelShown ? "flex" : "none",
              }}
            >
              <h2 className="text-center text-lg font-bold">your stats</h2>
              <p className="text-center text-sm font-normal text-black">
                {`do not forget to come here every day, for a few minutes at least - or else ur gonna be another disappointment for this world...`}
              </p>
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
