"use client";
import * as React from "react";
import { useState, useEffect } from "react";

function createUser(email: string, pswrd: string) {
  let response = fetch(
    `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}:3000/onboarding/register`,
    {
      method: "post",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: email, password: pswrd }),
    }
  );
  response.then((value) => {
    if (value.status == 201) {
      alert("CREATED ACCOUNT");
    } else if (value.status == 409) {
      alert("USER ALREADY EXISTS");
    }
  });
}
export default function LoginPage() {
  function testing(FormData: any) {
    createUser(FormData.get("email"), FormData.get("password"));
  }

  return (
    <div className="bg-gradient-to-r from-voodoo-500 h-screen flex flex-col justify-center">
      <h1 className="self-center text-6xl m-5 mb-9">Binkedin </h1>
      <form
        className="bg-voodoo-700  mx-9 flex flex-col rounded-md  lg:mx-64 lg:px-4"
        action={testing}
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
          Submit
        </button>
      </form>
    </div>
  );
}
