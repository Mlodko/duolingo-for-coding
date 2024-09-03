import Link from "next/link";
import React, { useState } from "react";
import type { Tab } from "./BottomBar";
import { useBottomBarItems } from "./BottomBar";
import type { LoginScreenState } from "./LoginScreen";
import { LoginScreen } from "./LoginScreen";
import _mainLogo from "../../public/logo.svg"
import type { StaticImageData } from "next/image";
import { UserLogOut } from "~/utils/backendUtils";
import { currentUser } from "~/utils/userData";

export const LeftBar = ({ selectedTab }: { selectedTab: Tab | null }) => {
  const loggedIn = () => {return currentUser.loggedIn};
  const logOut = UserLogOut;

  const [moreMenuShown, setMoreMenuShown] = useState(false);
  const [loginScreenState, setLoginScreenState] =
    useState<LoginScreenState>("HIDDEN");

  const mainLogo = _mainLogo as StaticImageData;

  const bottomBarItems = useBottomBarItems();

  return (
    <>
      <nav className="fixed bottom-0 left-0 top-0 flex-col gap-5 border-r-2 border-pink-ish bg-black p-3 md:flex lg:w-64 lg:p-5">
        <div className="flex">
          <img src={mainLogo.src} alt="main logo" height={90} width={90}/>
          <Link
            href="/"
            className="mb-5 ml-3 mt-5 hidden text-3xl font-bold text-pink-ish lg:block"
          >
            code samurai
          </Link>
        </div>
        <ul className="flex flex-col items-stretch gap-3">
          {bottomBarItems.map((item) => {
            return (
              <li key={item.href} className="flex flex-1">
                {item.name === selectedTab ? (
                  <Link
                    href={item.href}
                    className="flex grow items-center gap-3 rounded-xl border-2 border-pink-ish bg-dark-purple text-white px-3 py-2 text-sm font-bold"
                  >
                    <span className="lg:not-sr-only">{item.name}</span>
                  </Link>
                ) : (
                  <Link
                    href={item.href}
                    className="flex grow items-center gap-3 rounded-xl px-3 py-2 text-sm font-bold text-white hover:bg-pink-ish"
                  >
                    <span className="lg:not-sr-only">{item.name}</span>
                  </Link>
                )}
              </li>
            );
          })}
          <div
            className="relative flex grow cursor-default items-center gap-3 rounded-xl px-3 py-2 font-bold text-white hover:bg-pink-ish"
            onClick={() => setMoreMenuShown((x) => !x)}
            onMouseLeave={() => setMoreMenuShown(false)}
            role="button"
            tabIndex={0}
          >
            <span className="text-sm lg:inline">other</span>
            <div
              className={[
                "absolute left-full top-[-10px] min-w-[300px] rounded-2xl border-2 border-white bg-dark-purple text-left text-white",
                moreMenuShown ? "" : "hidden",
              ].join(" ")}
            >
              <div className="flex flex-col border-white py-3">
                {!loggedIn() && (
                  <button
                    className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                    onClick={() => setLoginScreenState("SIGNUP")}
                  >
                    make a profile
                  </button>
                )}
                <Link
                  className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                  href={loggedIn() ? "/settings/account" : "/settings/sound"}
                >
                  settings
                </Link>
                <Link
                  className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                  href="https://github.com/Mlodko/duolingo-for-coding"
                >
                  help
                </Link>
                {!loggedIn() && (
                  <button
                    className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                    onClick={() => setLoginScreenState("LOGIN")}
                  >
                    sign in
                  </button>
                )}
                {loggedIn() && (
                  <Link
                    className="px-5 py-2 text-left hover:bg-pink-ish"
                    onClick={logOut}
                    href="/"
                  >
                    sign out
                  </Link>
                )}
              </div>
            </div>
          </div>
        </ul>
      </nav>
      <LoginScreen
        loginScreenState={loginScreenState}
        setLoginScreenState={setLoginScreenState}
      />
    </>
  );
};
