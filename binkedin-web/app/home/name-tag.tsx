"use client";
import { useState, useEffect } from "react";

export default function NameCard({ auth }: any) {
  {
    const [name, setName] = useState("not logged in");
    const [ps, setPs] = useState("paswrd null");

    useEffect(() => {
      {
        auth.then((values: any) => {
          {
            if (values[0] && values[1]) {
              {
                setName(values[0]?.value);
                setPs(values[1]?.value);
              }
            }
          }
        });
      }
    }, [auth]);

    return (
      <div>
        this is the home page, you are {name} and {ps}
      </div>
    );
  }
}
