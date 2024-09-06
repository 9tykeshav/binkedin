"use client";
import * as React from "react";
import { useState, useEffect } from "react";
import Post from "@/app/home/postCard";
export default function Posts({ auth }: any) {
  const [Posts, setPosts] = useState([{}]);
  useEffect(() => {
    console.log("AUTH : ", auth);
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}:3000/api/post?email=${auth[0]}`,
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
        resp.json().then((posts) => {
          setPosts(posts);
        });
      }
    });
  }, [auth]);

  return (
    <div>
      {Posts.map((post: Record<string, string>, index) => (
        <Post key={index} postData={post}></Post>
      ))}
    </div>
  );
}
