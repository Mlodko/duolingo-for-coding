import Link from "next/link";
import React, { useState } from "react";
import type { LoginScreenState } from "./LoginScreen";
import { useBoundStore } from "~/hooks/useBoundStore";

type BottomBarItem = {
  name: Tab;
  href: string;
};

export type Tab = "course" | "my account";

export const useBottomBarItems = () => {
  const loggedIn = useBoundStore((x) => x.loggedIn);

  const bottomBarItems: BottomBarItem[] = [
    {
      name: "course",
      href: "/course"
    },
    {
      name: "my account",
      href: loggedIn ? "/my-account" : "/my-account?sign-up",
    },
  ];

  return bottomBarItems;
};

export const BottomBar = ({ selectedTab }: { selectedTab: Tab | null }) => {
  const logOut = useBoundStore((x) => x.logOut);
  const [loginScreenState, setLoginScreenState] =
    useState<LoginScreenState>("HIDDEN");

  const bottomBarItems = useBottomBarItems();
  const loggedIn = useBoundStore((x) => x.loggedIn);
  const [moreMenuShown, setMoreMenuShown] = useState(false);


  return (
    <nav className="fixed bottom-0 left-0 right-0 z-20 text-white border-t-2 border-pink-ish bg-dark-purple md:hidden">
      <ul className="flex h-[88px]">
        {bottomBarItems.map((item) => {
          return (
            <li
              key={item.href}
              className="text-white font-bold flex flex-1 items-center justify-center"
            >
              <Link
                href={item.href}
                className={
                  item.name === selectedTab
                    ? "rounded-xl border-2 border-pink-ish bg-darker-purple py-2 px-3"
                    : "rounded-xl hover:bg-white hover:text-darker-purple hover:border-2 hover:border-darker-purple py-2 px-3"
                }
              >
                {item.name}
                <span className="sr-only">{item.name}</span>
              </Link>
            </li>
          );
        })}
          <div
            className="text-white font-bold flex flex-1 items-center justify-center"
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
                {!loggedIn && (
                  <button
                    className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                    onClick={() => setLoginScreenState("SIGNUP")}
                  >
                    make a profile
                  </button>
                )}
                <Link
                  className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                  href={loggedIn ? "/settings/account" : "/settings/sound"}
                >
                  settings
                </Link>
                <Link
                  className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                  href="https://github.com/Mlodko/duolingo-for-coding"
                >
                  help
                </Link>
                {!loggedIn && (
                  <button
                    className="px-5 py-2 rounded-xl text-left hover:bg-pink-ish"
                    onClick={() => setLoginScreenState("LOGIN")}
                  >
                    sign in
                  </button>
                )}
                {loggedIn && (
                  <button
                    className="px-5 py-2 text-left hover:bg-pink-ish"
                    onClick={logOut}
                  >
                    sign out
                  </button>
                )}
              </div>
            </div>
          </div>

      </ul>
    </nav>
  );
};
