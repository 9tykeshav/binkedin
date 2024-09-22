import * as React from "react";
import SearchBar from "@/app/profile/[username]/searchBar";
import ProfileCard from "@/app/profile/[username]/profileCard";
import Posts from "@/app/profile/[username]/posts";
import { cookies } from "next/headers";
export default function UserProfilePage() {
  const cookieStore = cookies();
  const email = cookieStore.get("email")?.value;
  const password = cookieStore.get("psrwd")?.value;

  return (
    <div>
      <SearchBar></SearchBar>
      <ProfileCard></ProfileCard>
      <Posts auth={[email, password]}></Posts>
    </div>
  );
}
