"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
const cookieStore = cookies();
export async function doesCookiesExists() {
  const email = cookieStore.get("email");
  const password = cookieStore.get("psrwd");
  console.log(email, password); //FOR DEBBUGGING
  if (email && password) {
    // REDIRCT TO HOME

    return true;
  }
}
export async function getAuthCookies() {
  const email = cookieStore.get("email");
  const password = cookieStore.get("psrwd");
  // await doesCookiesExists(); //for debbugging
  return [email, password];
}
export async function createAuthCookies(email: string, pswrd: string) {
  cookieStore.set("email", email);
  cookieStore.set("psrwd", pswrd);
  // REDIRECT TO HOME
}
