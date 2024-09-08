import * as React from "react";

export default function PostCardSkeleton() {
  return (
    <div>
      <div className="bg-voodoo-100 rounded-lg m-3 p-2 animate-pulse">
        <div className="mx-5 my-3 text-xl w-24 h-5 rounded-xl bg-slate-400" />
        <div className="mx-3 m-2 ">
          <div className="mx-5 my-3 text-xl w-36 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-56 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-40 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-68 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-56 h-3 rounded-xl bg-slate-400" />
        </div>
        <div className="mx-5 my-3 text-xl w-56 h-46 rounded-xl bg-slate-400" />
        <div className="bg-voodoo-200 rounded-lg m-3">
          <button className="mx-3 my-2 bg-voodoo-500 rounded-md px-16 p-4"></button>
          <button className="mx-3 my-2  bg-voodoo-500 rounded-md px-16 p-4"></button>
        </div>
      </div>

      <div className="bg-voodoo-100 rounded-lg m-3 p-2 animate-pulse">
        <div className="mx-5 my-3 text-xl w-24 h-5 rounded-xl bg-slate-400" />
        <div className="mx-3 m-2 ">
          <div className="mx-5 my-3 text-xl w-36 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-56 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-40 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-68 h-3 rounded-xl bg-slate-400" />
          <div className="mx-5 my-3 text-xl w-56 h-3 rounded-xl bg-slate-400" />
        </div>
        <img src="/test-image.png" className="p-3" />
        <div className="bg-voodoo-200 rounded-lg m-3">
          <button className="mx-3 my-2 bg-voodoo-500 rounded-md px-16 p-4"></button>
          <button className="mx-3 my-2  bg-voodoo-500 rounded-md px-16 p-4"></button>
        </div>
      </div>
    </div>
  );
}
