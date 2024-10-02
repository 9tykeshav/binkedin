import * as React from "react";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import NameCard from "@/app/home/name-tag";
import Posts from "@/app/home/posts";
import BottomBar from "@/app/home/bottomBar";
import RequestBox from "@/app/connections/requestbox";
export default function connectionPage() {
  const cookieStore = cookies();
  const email = cookieStore.get("email")?.value;
  const password = cookieStore.get("psrwd")?.value;

  if (email == undefined || password == undefined) {
    redirect("/login");
  }

  return (
    <div className="bg-voodoo-50 h-screen">
      <div className="sticky top-0 z-10 p-1 bg-voodoo-50">
        <NameCard auth={[email, password]}></NameCard>
      </div>

      <div className="lg:px-60 ">
        <div>followers</div>
        <div>
          <RequestBox auth={[email, password]}></RequestBox>
        </div>
      </div>
      <div className="fixed bottom-0 z-10 p-1 bg-voodoo-50 w-screen">
        <BottomBar auth={[email, password]}></BottomBar>
      </div>
    </div>
  );
}
