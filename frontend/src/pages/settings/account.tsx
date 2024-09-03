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
  const _username = (currentUser.username === null ? " " : currentUser.username.slice(0));
  const _email = (currentUser.email === null ? " " : currentUser.email.slice(0));
  const _phone = (currentUser.phone === null ? " " : currentUser.phone.slice(0));
  const _bio = (currentUser.bio === null ? " " : currentUser.bio.slice(0));

  const _newUsername = _username.slice(0);
  const _newEmail = _email.slice(0);
  const _newPhone = _phone.slice(0);
  const _newBio = _bio.slice(0);

  const accountOptions = [
    { title: "username:", value: _newUsername, reference: _username },
    { title: "email:", value: _newEmail, reference: _email },
    { title: "phone:", value: _newPhone, reference: _phone },
    { title: "bio:", value: _newBio, reference: _bio },
  ];
  
  return (
    <div className="min-h-screen flex flex-1 flex-col text-white bg-darker-purple">
      <TopBar />
      <LeftBar selectedTab={null} />
      <BottomBar selectedTab={null} />
      <div className="flex-1 flex flex-col gap-5 px-4 py-20 sm:py-10 md:pl-28 lg:pl-72">
        <div className="mx-auto flex w-full max-w-xl items-center justify-between lg:max-w-4xl">
          <h1 className="font-bold text-gray-800 sm:text-2xl">
            my account
          </h1>
          <button
            className="rounded-2xl border-b-4 border-pink-ish bg-dark-purple px-5 py-3 font-bold uppercase text-white transition hover:bg-pink-ish hover:border-dark-purple disabled:hover:bg-dark-purple disabled:text-darker-purple disabled:border-pink-ish"
            onClick={() => {
              currentUser.username = _newUsername;
              currentUser.email = _newEmail;
              currentUser.phone = _newPhone;
              currentUser.bio = _newBio;

              // TODO: CALL USER DATA UPDATE BABYYY
            }}
            disabled={_username === _newUsername && _email === _newEmail && _phone === _newPhone && _bio === _newBio}
          >
            save changes
          </button>
        </div>
        <div className="flex h-full justify-center gap-12">
          <div className="flex w-full max-w-xl flex-col gap-8">
            {accountOptions.map(({ title, value, reference }) => {
              return (
                <div
                  key={title}
                  className="flex flex-col items-stretch justify-between gap-2 sm:flex-row sm:items-center sm:justify-center sm:gap-10 sm:pl-10"
                >
                  <div className="font-bold sm:w-1/6">{title}</div>
                  <input
                    className="text-black font-light grow rounded-2xl border-2 p-4 py-2"
                    value={reference}
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
