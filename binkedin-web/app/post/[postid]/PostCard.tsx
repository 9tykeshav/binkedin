"use client";

import * as React from "react";
import { useParams } from "next/navigation";
import PostCardSkeleton from "@/app/home/postCardSkeleton";

export default function Post({ auth }: any) {
  const [PostData, setPostData] = React.useState<any>({});
  const [IsLoading, setIsLoading] = React.useState(true);
  const params = useParams<{ postid: string }>();

  React.useEffect(() => {
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/post?postid=${params["postid"]}`,
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
      if (resp.status == 302) {
        resp.json().then((post) => {
          setPostData(post);
          setIsLoading(false);
        });
      }
    });
  }, [auth]);

  if (IsLoading) {
    return <PostCardSkeleton />;
  }

  return (
    <div>
      <div className="bg-voodoo-100 rounded-lg m-1 p-2 border-2 border-voodoo-900 mb-5">
        <h1 className="mx-5 my-3 text-xl">{PostData["user_email"]} </h1>
        <div className="mx-3 m-2 ">{PostData["caption"]}</div>
        {PostData["image_url"] ? (
          <img src={"/test-image.png"} className="p-3 " />
        ) : null}

        <div className="bg-voodoo-200 rounded-lg m-3">
          <button className="mx-3 my-2 p-1 bg-voodoo-500 rounded-md px-6 transform  hover:transition-all hover:scale-110">
            likes : {PostData["post_like_count"]}
          </button>
          <button className="mx-3 my-2 p-1 bg-voodoo-500 rounded-md px-6 transform  hover:transition-all hover:scale-110">
            comments: {PostData["post_comment_count"]}
          </button>
        </div>
      </div>

      <div></div>
    </div>
  );
}
