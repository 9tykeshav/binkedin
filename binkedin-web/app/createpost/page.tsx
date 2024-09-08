"use client";
import * as React from "react";
import { getAuthCookies } from "@/app/actions/manageCookie";
import { useRouter } from "next/navigation";
export default function CreatePost() {
  const [authDetail, setAuth] = React.useState<(string | undefined)[]>([]);
  const [data, setData] = React.useState<FormData>();
  const router = useRouter();
  function HandleCreatePost(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    console.log(data);
    setData(data);

    let auth = getAuthCookies();
    auth.then((v) => {
      let values = [v[0]?.value, v[1]?.value];
      setAuth(values);
      console.log(values);
    });
  }
  React.useEffect(() => {
    if (authDetail[0] && authDetail[1]) {
      let response = fetch(
        `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/post`,
        {
          method: "post",
          headers: { email: authDetail[0], password: authDetail[1] },
          body: data,
        }
      );
      router.push("/home");
    }
  }, [authDetail]);
  return (
    <div className="bg-voodoo-50 h-screen">
      <div className="bg-voodoo-500 h-12 flex flex-row items-center m-1 rounded-xl">
        <h1 className=" text-xl m-2">Create Post</h1>
      </div>
      <form
        className="bg-voodoo-400 m-2 p-1 rounded-lg flex flex-col"
        onSubmit={HandleCreatePost}
        autoComplete="off"
      >
        <label className="m-2"> Caption </label>
        <input
          className="m-1 rounded-lg h-48 p-1"
          type="text"
          placeholder="Enter captions"
          id="caption"
          name="caption"
        ></input>
        <input
          className="rounded-lg bg-voodoo-300 m-1 p-1"
          type="file"
          name="image"
          id="image"
        />
        <button
          className="bg-voodoo-700 m-3 p-1 rounded-xl h-12 text-2xl outline outline-voodoo-800  transform hover:scale-105"
          type="submit"
        >
          Create post
        </button>
      </form>
    </div>
  );
}
