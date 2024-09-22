"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
export default function NameCard({ auth }: any) {
  const name = auth[0];
  const ps = auth[1];
  const router = useRouter();

  return (
    <div>
      <div className="flex h-12 bg-voodoo-800 flex-row items-center rounded-lg m-1  ">
        <div className="mx-2 font-bold text-slate-100 outline-4 outline-black">
          {name} {ps}
        </div>
        <div className=" ">
          <input
            className=" w-48 lg:w-96 mx-2 rounded-2xl px-3"
            type="text"
            placeholder="Search"
          />
        </div>
        <button
          className="bg-voodoo-600  m-1 p-1 rounded-lg w-8"
          onClick={() => router.push("/createpost")}
        >
          +
        </button>
      </div>
    </div>
  );
}
