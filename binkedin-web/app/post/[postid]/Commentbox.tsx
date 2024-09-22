"use client";

import * as React from "react";
import { useParams } from "next/navigation";
import { useRouter } from "next/navigation";

export default function Commentbox({ auth }: any) {
  const router = useRouter();
  const [Comments, setComments] = React.useState([{}]);
  const [isLoading, setIsLoading] = React.useState(true);
  let ref = React.useRef("");
  const params = useParams<{ postid: string }>();

  function HandleMakeComment(formData: FormData) {
    formData.append("post_id", params["postid"]);
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/comment`,
      {
        method: "post",
        headers: {
          "Content-Type": "application/json",
          email: auth[0],
          password: auth[1],
        },
        body: JSON.stringify(Object.fromEntries(formData)),
      }
    );

    router.refresh();
  }

  React.useEffect(() => {
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/comment?postid=${params["postid"]}`,
      {
        method: "get",
        headers: {
          "Content-Type": "application/json",
          email: auth[0],
          password: auth[1],
        },
      }
    );
    response.then((resp) => {
      if (resp.status == 200) {
        resp.json().then((c) => {
          setComments(c);
        });
      }
    });
  }, [auth]);

  return (
    <>
      <div className="pb-12">
        {Comments.map((comment: Record<string, string>, index) => (
          <div
            key={index}
            className="bg-voodoo-100  p-1 rounded-lg  border-2 border-voodoo-500 mt-1 mb-1 m-1"
          >
            <div className="">
              <h1 className="m-1 text-xl">{comment["author_email"]}</h1>
              <p className="m-1 ">{comment["content"]}</p>
            </div>
          </div>
        ))}
      </div>

      <div className="fixed bottom-0 w-full lg:w-3/5 lg:ml-62 flex flex-row justify-center bg-voodoo-50">
        <form action={HandleMakeComment} className="w-full">
          <input
            className="w-full   p-2 rounded-lg border-2 border-voodoo-400"
            type="text"
            name="content"
            id=""
            placeholder="Comment..... something"
          />
        </form>
      </div>
    </>
  );
}
