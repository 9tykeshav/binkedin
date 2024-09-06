import * as React from "react";

export default function Post({ postData }: any) {
  console.log(postData["post_id"]);

  return (
    <div className="bg-voodoo-100 rounded-lg m-3 p-2">
      <h1 className="mx-5 my-3 text-xl">Keshav</h1>
      <div className="mx-3 m-2 ">
        It is a long established fact that a reader will be distracted by the
        readable content of a page when looking at its layout. The point of
        using Lorem Ipsum is that it has a more-or-less normal distribution of
        letters, as opposed to using 'Content here, content here', making it
        look like readable English. Many desktop publishing packages and web
        page editors now use Lorem Ipsum as their default model text, and a
        search for 'lorem ipsum' will uncover many web sites still in their
        infancy
      </div>
      <img src="/test-image.png" className="p-3" />
      <div className="bg-voodoo-200 rounded-lg m-3">
        <button className="mx-3 my-2 p-1 bg-voodoo-500 rounded-md px-8">
          likes 55
        </button>
        <button className="mx-3 my-2 p-1 bg-voodoo-500 rounded-md px-8">
          comments: 55
        </button>
      </div>
    </div>
  );
}
