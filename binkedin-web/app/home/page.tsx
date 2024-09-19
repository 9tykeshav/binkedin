import * as React from "react";
import { getAuthCookies } from "@/app/actions/manageCookie";
import NameCard from "@/app/home/name-tag";
import Posts from "@/app/home/posts";
import { cookies } from "next/headers";
import PostCardSkeleton from "@/app/home/postCardSkeleton";
import { redirect } from "next/navigation";
export default function Home() {
  const cookieStore = cookies();
  const email = cookieStore.get("email")?.value;
  const password = cookieStore.get("psrwd")?.value;

  if (email == undefined || password == undefined) {
    redirect("/login");
  }

  return (
    <div className="bg-voodoo-50">
      <div className="sticky top-0 z-10 p-1 bg-voodoo-50">
        <NameCard auth={[email, password]}></NameCard>
      </div>

      <div className="lg:px-60 ">
        <Posts auth={[email, password]}></Posts>
      </div>
    </div>
  );
}
