import type { NextPage } from "next";
import { BottomBar } from "~/components/BottomBar";
import { LeftBar } from "~/components/LeftBar";
import {
  EditPencilSvg,
  EmptyFireSvg,
  FireSvg,
} from "~/components/Svgs";
import Link from "next/link";
import { Flag } from "~/components/Flag";
import { useBoundStore } from "~/hooks/useBoundStore";
import { useEffect } from "react";
import { useRouter } from "next/router";
import _lvlUpStar from "../../public/drawing-1.svg"
import type { StaticImageData } from "next/image";
import { currentUser } from "~/utils/userData";

const lvlUpStar = _lvlUpStar as StaticImageData;

const myAccount: NextPage = () => {
  return (
    <div className="bg-darker-purple text-white">
      <LeftBar selectedTab="my account" />
      <div className="bg-darker-purple flex justify-center gap-3 pt-14 md:ml-24 lg:ml-64 lg:gap-12">
        <div className="flex w-full max-w-4xl flex-col gap-5 p-5">
          <ProfileTopSection />
          <ProfileStatsSection />
        </div>
      </div>
      <div className="pt-[90px]"></div>
      <BottomBar selectedTab="my account" />
    </div>
  );
};

export default myAccount;


const ProfileTopSection = () => {
  const router = useRouter();
  const loggedIn = currentUser.loggedIn;
  const username = currentUser.username;
  const id = currentUser.id;
  const language = useBoundStore((x) => x.language);

  if (typeof (id) === 'undefined')

  useEffect(() => {
    if (!loggedIn) {
      void router.push("/");
    }
  }, [loggedIn, router]);

  return (
    <section className="bg-darker-purple flex flex-row-reverse border-b-2 border-gray-200 pb-8 md:flex-row md:gap-8">
      <div className="flex h-20 w-20 items-center justify-center rounded-full border-2 border-dashed border-gray-400 text-3xl font-bold text-white md:h-44 md:w-44 md:text-7xl">
        {id!.charAt(0).toUpperCase()}
      </div>
      <div className="flex grow flex-col justify-between gap-3">
        <div className="flex flex-col gap-2">
          <div>
            <h1 className="text-2xl text-white font-bold">{username}</h1>
            <div className="text-sm text-white">{id}</div>
          </div>
        </div>
        <div className="text-white flex flex-col justify-between gap-3">
          <h2 className="font-bold">your courses: </h2>
          <Flag language={language} width={30} />
        </div>
      </div>
      <Link
        href="/settings/account"
        className="hidden items-center gap-2 self-start rounded-2xl border-b-4 border-pink-ish bg-dark-purple px-6 py-4 font-bold text-white transition hover:bg-pink-ish hover:border-dark-purple md:flex"
      >
        <EditPencilSvg />
        edit profile
      </Link>
    </section>
  );
};

const ProfileStatsSection = () => {
  const streak = useBoundStore((x) => x.streak);
  const totalXp = 125;

  return (
    <section>
      <h2 className="text-white mb-5 text-2xl font-bold">your stats</h2>
      <div className="grid grid-cols-2 gap-3">
        <div className="flex gap-2 rounded-2xl border-2 border-white p-2 md:gap-3 md:px-6 md:py-4">
          {streak === 0 ? <EmptyFireSvg /> : <FireSvg />}
          <div className="flex flex-col">
            <span
              className={[
                "text-xl font-bold",
                streak === 0 ? "text-pink-ish" : "text-white",
              ].join(" ")}
            >
              {streak}
            </span>
            <span className="text-sm text-white md:text-base">
              your streak (in days)
            </span>
          </div>
        </div>
        <div className="flex gap-2 rounded-2xl border-2 border-white p-2 md:gap-3 md:px-6 md:py-4">
          <img src={lvlUpStar.src} height={35} width={35}/>
          <div className="flex flex-col">
            <span className="text-xl text-white font-bold">{totalXp}</span>
            <span className="text-sm text-white md:text-base">your xp</span>
          </div>
        </div>
      </div>
    </section>
  );
};