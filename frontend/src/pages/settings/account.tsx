import type { NextPage } from "next";
import React, { useState } from "react";
import { BottomBar } from "~/components/BottomBar";
import { LeftBar } from "~/components/LeftBar";
import { TopBar } from "~/components/TopBar";
import { SettingsRightNav } from "~/components/SettingsRightNav";
import { useBoundStore } from "~/hooks/useBoundStore";
import { currentUser } from "~/utils/userData";

const Account: NextPage = () => {
  /*
    TODO:
        motherfokin user data update

  */
  var _username = currentUser.username!;
  var _email = currentUser.email!;
  var _phone = currentUser.phone!;
  var _bio = currentUser.bio!;

  const accountOptions = [
    { title: "username:", value: _username },
    //{ title: "user ID:", value: localUsername, setValue: setLocalUsername }, //< this is gonna be listed, but not immutable
    { title: "email:", value: _email },
    { title: "phone:", value: _phone },
    { title: "bio:", value: _bio },
  ];
  
  return (
    <div className="text-white bg-darker-purple">
      <TopBar />
      <LeftBar selectedTab={null} />
      <BottomBar selectedTab={null} />
      <div className="mx-auto flex flex-col gap-5 px-4 py-20 sm:py-10 md:pl-28 lg:pl-72">
        <div className="mx-auto flex w-full max-w-xl items-center justify-between lg:max-w-4xl">
          <h1 className="text-lg font-bold text-gray-800 sm:text-2xl">
            my account
          </h1>
          <button
            className="rounded-2xl border-b-4 border-pink-ish bg-dark-purple px-5 py-3 font-bold uppercase text-white transition hover:bg-pink-ish hover:border-dark-purple disabled:hover:bg-dark-purple disabled:text-darker-purple disabled:border-pink-ish"
            onClick={() => {
              currentUser.username = _username;
              currentUser.email = _email;
              currentUser.phone = _phone;
              currentUser.bio = _bio;

              // TODO: CALL USER DATA UPDATE BABYYYY
            }}
            disabled={_username === currentUser.username && _email === currentUser.email && _phone === currentUser.phone && _bio === currentUser.bio}
          >
            save changes
          </button>
        </div>
        <div className="flex justify-center gap-12">
          <div className="flex w-full max-w-xl flex-col gap-8">
            {accountOptions.map(({ title, value }) => {
              return (
                <div
                  key={title}
                  className="flex flex-col items-stretch justify-between gap-2 sm:flex-row sm:items-center sm:justify-center sm:gap-10 sm:pl-10"
                >
                  <div className="font-bold sm:w-1/6">{title}</div>
                  <input
                    className="text-black font-light grow rounded-2xl border-2 p-4 py-2"
                    value={value}
                    onChange={(e) => { value = e.target.value }}
                  />
                </div>
              );
            })}
          </div>
          <SettingsRightNav selectedTab="my account" />
        </div>
      </div>
    </div>
  );
};

export default Account;
