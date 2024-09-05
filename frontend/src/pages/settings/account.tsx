import type { NextPage } from "next";
import { useRouter } from "next/router";
import React, { useState } from "react";
import { useRef } from "react";
import { BottomBar } from "~/components/BottomBar";
import { LeftBar } from "~/components/LeftBar";
import { TopBar } from "~/components/TopBar";
import { SettingsRightNav } from "~/components/SettingsRightNav";
import { useBoundStore } from "~/hooks/useBoundStore";
import { currentUser } from "~/utils/userData";
import { UserDataUpdate } from "~/utils/backendUtils";
import Link from "next/link";

const Account: NextPage = () => {
  /*
    TODO:
        motherfokin user data update

  */

  const [localUsername, setLocalUsername] = useState(currentUser.username);
  const [localEmail, setLocalEmail] = useState(currentUser.email);
  const [localPhone, setLocalPhone] = useState(currentUser.phone);
  const [localBio, setLocalBio] = useState(currentUser.bio);

  const usernameInput = useRef<null | HTMLInputElement>(null);
  const emailInput = useRef<null | HTMLInputElement>(null);
  const phoneInput = useRef<null | HTMLInputElement>(null);
  const bioInput = useRef<null | HTMLInputElement>(null);

  const updateUserData = () => {
    updateDataFails(false);

    UserDataUpdate().then((ifSucceeded) => {
      if (!ifSucceeded) {
        updateDataFails(true);
      }
    });     
  }

  const [ifUpdateDataFailed, updateDataFails] = useState(false);

  const accountOptions = [
    { title: "username:", placeholder: currentUser.username, ref: usernameInput, onChange: setLocalUsername },
    { title: "email:", placeholder: currentUser.email, ref: emailInput, onChange: setLocalEmail },
    { title: "phone:", placeholder: currentUser.phone, ref: phoneInput, onChange: setLocalPhone },
    { title: "bio:", placeholder: currentUser.bio, ref: bioInput, onChange: setLocalBio },
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
          <Link
            href="/settings/account"
          >
            <button
              className="rounded-2xl border-b-4 border-pink-ish bg-dark-purple px-5 py-3 font-bold uppercase text-white transition hover:bg-pink-ish hover:border-dark-purple disabled:hover:bg-dark-purple disabled:text-darker-purple disabled:border-pink-ish"
              onClick={() => {
                if (usernameInput.current?.value.trim()! !== "")
                  currentUser.username = usernameInput.current?.value.trim()!;
                if (emailInput.current?.value.trim()! !== "")
                  currentUser.email = emailInput.current?.value.trim()!;
                if (phoneInput.current?.value.trim()! !== "")
                  currentUser.phone = phoneInput.current?.value.trim()!;
                if (bioInput.current?.value.trim()! !== "")
                  currentUser.bio = bioInput.current?.value.trim()!;

                updateUserData();

                accountOptions.map(({ ref, onChange }) => {
                  ref.current!.value = "";
                  onChange("");
                });
              }}
              disabled={ (currentUser.username === localUsername || localUsername === "")
                      && (currentUser.email === localEmail || localEmail === "")
                      && (currentUser.phone === localPhone || localPhone === "")
                      && (currentUser.bio === localBio || localBio === "")
              } 
            >
              save changes
            </button>
          </Link>
        </div>
        <div className="flex h-full justify-center gap-12">
          <div className="flex w-full max-w-xl flex-col gap-8">
            {accountOptions.map(({ title, placeholder, ref, onChange  }) => {
              return (
                <div
                  key={title}
                  className="flex flex-col items-stretch justify-between gap-2 sm:flex-row sm:items-center sm:justify-center sm:gap-10 sm:pl-10"
                >
                  <div className="font-bold sm:w-1/6">{title}</div>
                  <input
                    className="text-black font-light grow rounded-2xl border-2 p-4 py-2"
                    placeholder={placeholder}
                    ref={ref}
                    onChange={(e) => onChange(e.target.value)}
                  />
                </div>
              );
            })}
            {ifUpdateDataFailed && (
              <p className="text-center text-xs leading-5 text-white">
                sorry, couldn't update yo data :{"("}, try again l8r
              </p>
            )}
          </div>
          <SettingsRightNav selectedTab="my account" />
        </div>
      </div>
    </div>
  );
};

export default Account;
