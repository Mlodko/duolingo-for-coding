import Link from "next/link";
import React from "react";
import { useBoundStore } from "~/hooks/useBoundStore";
import { currentUser } from "~/utils/userData";

type SettingsTitle = ReturnType<typeof useSettingsPages>[number]["title"];

const useSettingsPages = () => {
  const loggedIn = () => { return currentUser.loggedIn };
  return loggedIn()
    ? ([
        { title: "my account", href: "/settings/account" },
        { title: "sounds", href: "/settings/sound" },
      ] as const)
    : ([
        { title: "sounds", href: "/settings/sound" },
      ] as const);
};

export const SettingsRightNav = ({
  selectedTab,
}: {
  selectedTab: SettingsTitle;
}) => {
  const settingsPages = useSettingsPages();
  return (
    <div className="text-white hidden h-fit w-80 flex-col gap-1 rounded-2xl border-2 p-5 lg:flex">
      {settingsPages.map(({ title, href }) => {
        return (
          <Link
            key={title}
            href={href}
            className={[
              "rounded-2xl p-4 font-bold hover:bg-pink-ish",
              title === selectedTab ? "bg-dark-purple" : "",
            ].join(" ")}
          >
            {title}
          </Link>
        );
      })}
    </div>
  );
};
