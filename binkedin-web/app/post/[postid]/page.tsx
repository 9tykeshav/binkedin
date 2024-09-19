import * as React from "react";
import { getAuthCookies } from "@/app/actions/manageCookie";
import NameCard from "@/app/home/name-tag";
import Post from "@/app/post/[postid]/PostCard";
import { cookies } from "next/headers";
import Commentbox from "@/app/post/[postid]/Commentbox";
export default function Home() {
  const cookieStore = cookies();
  const email = cookieStore.get("email")?.value;
  const password = cookieStore.get("psrwd")?.value;

  return (
    <div className="bg-voodoo-50  ">
      <div className="bg-voodoo-50">
        <Post auth={[email, password]}></Post>
      </div>
      <div className="w-full h-full">
        <Commentbox auth={[email, password]}></Commentbox>
      </div>
    </div>
  );
}
