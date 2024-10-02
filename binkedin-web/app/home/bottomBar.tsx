"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
export default function BottomBar({ auth }: any) {
  const name = auth[0];
  const ps = auth[1];
  const router = useRouter();

  return (
    <div className="h-12 bg-voodoo-700">
      <div className="flex flex-row justify-evenly p-1">
        <div className=" bg-voodoo-400 p-1 m-1 rounded-lg">
          <button>connections</button>
        </div>
        <div className=" bg-voodoo-400 p-1 m-1 rounded-lg">
          <button onClick={() => router.push("/createpost")}>post</button>
        </div>
        <div className=" bg-voodoo-400 p-1 m-1 rounded-lg">
          <button onClick={() => router.push(`/profile/${auth[0]}`)}>
            profile
          </button>
        </div>
      </div>
    </div>
  );
}
