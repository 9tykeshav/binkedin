import * as React from "react";
import { getAuthCookies } from "@/app/actions/manageCookie";
import { RequestCookie } from "next/dist/compiled/@edge-runtime/cookies";
import NameCard from "@/app/home/name-tag";

export default function Home() {
  let auth = getAuthCookies();

  return (
    <div>
      <NameCard auth={auth}></NameCard>
    </div>
  );
}
