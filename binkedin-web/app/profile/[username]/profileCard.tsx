"use client";
import * as React from "react";
import { useParams } from "next/navigation";
export default function ProfileCard({
  username = useParams<{ username: string }>().username,
  followers = 500,
  following = 600,
}) {
  return (
    <div className="bg-voodoo-200 rounded-2xl">
      <div className="flex flex-col m-1 flex-wrap">
        <h1 className="m-1 text-2xl"> {username} </h1>
        <div className="flex flex-row justify-evenly">
          <div>
            <h1 className="m-1 mx-7">{followers}</h1>
          </div>
          <div>
            <h1 className="m-1 mx-7">124</h1>
          </div>
          <div>
            <h1 className="m-1 mx-7">124</h1>
          </div>
        </div>
      </div>
    </div>
  );
}
