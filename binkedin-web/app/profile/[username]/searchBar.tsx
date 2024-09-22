import * as React from "react";

export default function SearchBar() {
  return (
    <>
      <div className="h-14 bg-voodoo-300 flex flex-row items-center justify-evenly">
        <button
          className="bg-voodoo-500 h-9 w-9 rounded-xl mr-3 "
          type="button"
        ></button>
        <div className="flex flex-row">
          <input
            className=" w-64 h-2/4 rounded-3xl self-center mr-14 p-2 border-2 border-voodoo-500"
            type="text"
            name="usernameToSearch"
            id=""
            placeholder="username..."
          />
        </div>
      </div>
    </>
  );
}
