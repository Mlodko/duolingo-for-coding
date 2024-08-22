import { ChevronDownSvg } from "./Svgs";
import { useState } from "react";
import Link from "next/link";

export const LanguageDropDown = () => {
  const [languagesShown, setLanguagesShown] = useState(false);
  return (
    <div
      className="relative hidden cursor-pointer items-center md:flex"
      onMouseEnter={() => setLanguagesShown(true)}
      onMouseLeave={() => setLanguagesShown(false)}
      aria-haspopup={true}
      aria-expanded={languagesShown}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          setLanguagesShown((isShown) => !isShown);
        }
      }}
    >
      <span className="text-md">language: ENGLISH</span>{" "}
      <ChevronDownSvg />
      {languagesShown && (
        <ul className="absolute right-0 top-full grid w-[500px] grid-cols-2 rounded-2xl border-2 border-white-200 bg-dark-purple p-6 font-light text-white-600">
              <li>
                <Link
                  href={`https://youtu.be/AagRYDxrZ7k?feature=shared`}
                  tabIndex={0}
                  className="flex items-center gap-3 whitespace-nowrap rounded-xl p-3 hover:bg-pink-ish"
                >
                  placeholder ;3
                </Link>
              </li>
        </ul>
      )}
    </div>
  );
};
