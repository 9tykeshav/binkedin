import * as React from "react";
import { getAuthCookies } from "@/app/actions/manageCookie";
import NameCard from "@/app/home/name-tag";
import Posts from "@/app/home/posts";
import { cookies } from "next/headers";
export default function Home() {
  const cookieStore = cookies();
  const email = cookieStore.get("email")?.value;
  const password = cookieStore.get("psrwd")?.value;

  return (
    <div>
      <NameCard auth={[email, password]}></NameCard>
      <Posts auth={[email, password]}></Posts>
    </div>
  );
}
