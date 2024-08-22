import Link from "next/link";
import { CloseSvg } from "./Svgs";
import React, { useEffect, useRef, useState } from "react";
import { useBoundStore } from "~/hooks/useBoundStore";
import { useRouter } from "next/router";

export type LoginScreenState = "HIDDEN" | "LOGIN" | "SIGNUP";

export const useLoginScreen = () => {
  const router = useRouter();
  const loggedIn = useBoundStore((x) => x.loggedIn);
  const queryState: LoginScreenState = (() => {
    if (loggedIn) return "HIDDEN";
    if ("login" in router.query) return "LOGIN";
    if ("sign-up" in router.query) return "SIGNUP";
    return "HIDDEN";
  })();
  const [loginScreenState, setLoginScreenState] = useState(queryState);
  useEffect(() => setLoginScreenState(queryState), [queryState]);
  return { loginScreenState, setLoginScreenState };
};

export const LoginScreen = ({
  loginScreenState,
  setLoginScreenState,
}: {
  loginScreenState: LoginScreenState;
  setLoginScreenState: React.Dispatch<React.SetStateAction<LoginScreenState>>;
}) => {
  const router = useRouter();
  const loggedIn = useBoundStore((x) => x.loggedIn);
  const logIn = useBoundStore((x) => x.logIn);
  const setUsername = useBoundStore((x) => x.setUsername);
  const setName = useBoundStore((x) => x.setName);

  const nameInputRef = useRef<null | HTMLInputElement>(null);

  useEffect(() => {
    if (loginScreenState !== "HIDDEN" && loggedIn) {
      setLoginScreenState("HIDDEN");
    }
  }, [loginScreenState, loggedIn, setLoginScreenState]);

  const logInAndSetUserProperties = () => {
    const name =
      nameInputRef.current?.value.trim() || Math.random().toString().slice(2);
    const username = name.replace(/ +/g, "-");
    setUsername(username);
    setName(name);
    logIn();
    void router.push("/course");
  };

  return (
    <article
      className={[
        "fixed inset-0 z-30 flex flex-col bg-darker-purple p-7 transition duration-300",
        loginScreenState === "HIDDEN"
          ? "pointer-events-none opacity-0"
          : "opacity-100",
      ].join(" ")}
      aria-hidden={!loginScreenState}
    >
      <header className="flex flex-row-reverse justify-between sm:flex-row">
        <button
          className="flex text-white"
          onClick={() => setLoginScreenState("HIDDEN")}
        >
          <CloseSvg />
          <span className="sr-only">close</span>
        </button>
        <button
          className="hidden rounded-2xl border-2 border-b-4 border-white px-4 py-3 text-m font-bold text-white transition hover:bg-white hover:text-darker-purple sm:block"
          onClick={() =>
            setLoginScreenState((x) => (x === "LOGIN" ? "SIGNUP" : "LOGIN"))
          }
        >
          {loginScreenState === "LOGIN" ? "sign up" : "login"}
        </button>
      </header>
      <div className="bg-darker-purple rounded-3xl text-white flex grow items-center justify-center">
        <div className="flex w-full flex-col gap-5 sm:w-96">
          <h2 className="text-center text-2xl font-bold text-white">
            {loginScreenState === "LOGIN" ? "log in" : "create your profile"}
          </h2>
          <div className="flex flex-col gap-2 text-black">
            <input
              className="grow rounded-2xl px-4 py-3"
              placeholder={
                loginScreenState === "LOGIN"
                  ? "email or username"
                  : "email"
              }
            />
          </div>
          <div className="flex flex-col gap-2 text-black">
            <div className="relative flex grow">
              <input
                className="grow rounded-2xl px-4 py-3"
                placeholder="password"
                type="password"
              />
              {loginScreenState === "LOGIN" && (
                <div className="absolute bottom-0 right-0 top-0 flex items-center justify-center pr-5">
                  <Link
                    className="text-black hover:text-pink-ish"
                    href="/forgot-password"
                  >
                    forgot password?
                  </Link>
                </div>
              )}
            </div>
          </div>
          {loginScreenState === "SIGNUP" && (
              <>
                <div className="relative flex grow">
                  <input
                    className="text-black grow rounded-2xl px-4 py-3"
                    placeholder="your username"
                    ref={nameInputRef}
                  />
                </div>
              </>
            )}

          {loginScreenState === "SIGNUP" && (
            <> 
             <div className="relative flex grow">
                <input
                  className="text-black grow rounded-2xl border-2 border-gray-200 bg-gray-50 px-4 py-3"
                  placeholder="your age"
                />
              </div>
            </>          
          )}

          <button
            className="rounded-2xl border-b-4 border-darker-purple bg-dark-purple py-3 font-bold text-white transition hover:bg-pink-ish"
            onClick={logInAndSetUserProperties}
          >
            {loginScreenState === "LOGIN" ? "log in" : "create account"}
          </button>
          <div className="flex items-center">
            <div className="h-[2px] grow bg-white"></div>
            <div className="h-[2px] grow bg-white"></div>
          </div>
          <p className="text-center text-xs leading-5 text-white">
            if you sign up here, you agree to the{" "}
            <Link
              className="font-bold hover:text-pink-ish"
              href="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
            >
              terms
            </Link>
              {" "}and{" "}
            <Link
              className="font-bold hover:text-pink-ish"
              href="https://youtu.be/WKXh8w-tvDQ?feature=shared"
            >
              privacy policy
            </Link>
            .
          </p>
          <p className="text-center text-xs leading-5 text-white">
            the site is protected by reCAPTCHA, just so you know
          </p>
          <p className="block text-center sm:hidden">
            <span className="text-sm font-bold text-white">
              {loginScreenState === "LOGIN"
                ? "no account?"
                : "have an account?"}
            </span>{" "}
            <button
              className="text-sm font-bold uppercase text-white"
              onClick={() =>
                setLoginScreenState((x) => (x === "LOGIN" ? "SIGNUP" : "LOGIN"))
              }
            >
              {loginScreenState === "LOGIN" ? "sign up" : "log in"}
            </button>
          </p>
        </div>
      </div>
    </article>
  );
};