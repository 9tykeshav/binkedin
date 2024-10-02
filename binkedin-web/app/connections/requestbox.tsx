"use client";
import * as React from "react";

function handleAcceptReq() {}

export default function RequestBox({ auth }: any) {
  const [pendingReq, setPendingReqs] = React.useState([1, 2, 3]);

  React.useEffect(() => {
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/follow-request`,
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
        resp.json().then((req) => {
          setPendingReqs(req);
          //   setIsLoading(false);
        });
      }
    });
  }, [auth]);

  return (
    <div>
      {pendingReq.map((req: any, index) => (
        <Request
          username={req["follower_username"]}
          key={index}
          auth={auth}
        ></Request>
      ))}
    </div>
  );
}

function Request({ username, auth }: any): React.JSX.Element {
  function handleAcceptReq() {
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/follow-request/accept/${username}`,
      {
        method: "put",
        headers: {
          "Content-Type": "application/json",
          email: auth[0],
          password: auth[1],
        },
      }
    );
  }

  function handleRejectReq() {
    let response = fetch(
      `http://${process.env.NEXT_PUBLIC_IP_ADDR_FOR_SERVICES}/api/follow-request/reject/${username}`,
      {
        method: "put",
        headers: {
          "Content-Type": "application/json",
          email: auth[0],
          password: auth[1],
        },
      }
    );
  }
  return (
    <div>
      <div className="w-screen bg-voodoo-400 h-14  m-1 rounded-lg flex items-center justify-between">
        <div className=" text-xl m-1 flex">{username}</div>
        <div className="flex flex-row justify-evenly p-1 m-1">
          <div className="m-1 bg-voodoo-200 rounded-lg">
            <button onClick={handleAcceptReq}>a</button>
          </div>
          <div className="m-1 bg-voodoo-200 rounded-lg">
            <button onClick={handleRejectReq}>d</button>
          </div>
        </div>
      </div>
    </div>
  );
}
