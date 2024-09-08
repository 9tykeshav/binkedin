"use client";

import * as React from "react";
import { useRouter } from "next/navigation";
import {
  doesCookiesExists,
  createAuthCookies,
} from "@/app/actions/manageCookie";
import { AppRouterInstance } from "next/dist/shared/lib/app-router-context.shared-runtime";

function Login(email: string, pswrd: string, r: AppRouterInstance) {
  let response = fetch(
    `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/onboarding/login`,
    {
      method: "post",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: email, password: pswrd }),
    }
  );
  response.then((value) => {
    if (value.status == 200) {
      createAuthCookies(email, pswrd);
      alert("go to home page, cookies set");
      r.push("/home");
    } else if (value.status == 404) {
      // redirect to register
      alert(
        "You have not registered yet, redirecting you to registration page"
      );
      r.push("/register");
    } else {
      console.log("check netwrking tab ;)");
      value.text().then((e) => console.log(e));
    }
  });
}

export default function LoginPage() {
  let router = useRouter();
  function HandleLogin(FormData: any) {
    Login(FormData.get("email"), FormData.get("password"), router);
  }

  return (
    <div className="bg-gradient-to-r from-voodoo-500 h-screen flex flex-col justify-center">
      <h1 className="self-center text-6xl m-5 mb-9">Binkedin </h1>
      <form
        className="bg-voodoo-700  mx-9 flex flex-col rounded-md  lg:mx-64 lg:px-4"
        action={HandleLogin}
        autoComplete="off"
      >
        <label className="flex flex-col items-center">
          <h1>Enter your email:</h1>
          <input
            name="email"
            className="rounded-md border-t-rose-700"
            type="text"
            placeholder="example@1234.wires"
          />
        </label>

        <label className="flex flex-col items-center">
          Enter your password:
          <input
            name="password"
            className="rounded-md"
            type="text"
            placeholder="not your DOB"
          />
        </label>
        <button
          className="p-3 bg-voodoo-600 rounded-lg w-30 my-2 self-center hover:bg-voodoo-900"
          type="submit"
        >
          Login
        </button>
      </form>
    </div>
  );
}
