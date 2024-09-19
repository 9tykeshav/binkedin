import * as React from "react";

export default function Post({ postData }: any) {
  return (
    <div className="bg-voodoo-100 rounded-lg m-3 p-2">
      <h1 className="mx-5 my-3 text-xl">{postData["user_email"]} </h1>
      <div className="mx-3 m-2 ">{postData["caption"]}</div>
      {postData["image_url"] ? (
        <img src={"/test-image.png"} className="p-3 " />
      ) : null}

      <div className="bg-voodoo-200 rounded-lg m-3">
        <button className="mx-3 my-2 p-1 bg-voodoo-500 rounded-md px-6 transform  hover:transition-all hover:scale-110">
          likes : {postData["post_like_count"]}
        </button>
        <a href={`/post/${postData["post_id"]}`}>
          <button className="mx-3 my-2 p-1 bg-voodoo-500 rounded-md px-6 transform  hover:transition-all hover:scale-110">
            comments: {postData["post_comment_count"]}
          </button>
        </a>
      </div>
    </div>
  );
}
