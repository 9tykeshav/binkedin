"use client";
import * as React from "react";
import { useState, useEffect } from "react";
import Post from "@/app/home/postCard";
import PostCardSkeleton from "@/app/home/postCardSkeleton";
import { useParams } from "next/navigation";
export default function Posts({ auth }: any, username: string) {
  const [Posts, setPosts] = useState([]);
  const [isLoading, setIsLoading] = useState(true);
  const params = useParams<{ username: string }>();
  useEffect(() => {
    console.log("AUTH : ", auth);
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/homefeed?email=${params["username"]}`,
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
          setIsLoading(false);
        });
      }
    });
  }, [auth]);

  if (isLoading) {
    return <PostCardSkeleton />;
  }

  return (
    <div>
      {Posts.map((post: Record<string, string>, index) => (
        <Post key={index} postData={post}></Post>
      ))}
    </div>
  );
}
