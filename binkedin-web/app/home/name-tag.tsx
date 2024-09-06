"use client";
import { useState, useEffect } from "react";

export default function NameCard({ auth }: any) {
  const name = auth[0]?.value;
  const ps = auth[1]?.value;

  return (
    <div>
      <div className="flex h-12 bg-voodoo-800 flex-row items-center ">
        <div className="mx-2 font-bold text-slate-100 outline-4 outline-black">
          {name} {ps}
        </div>
        <div className="justify-self-end">
          <input
            className=" mx-2 rounded-md px-3 lg:w-96 lg:self-center"
            type="text"
            placeholder="Search"
          />
        </div>
      </div>
    </div>
  );
}
