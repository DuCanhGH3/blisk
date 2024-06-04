import { JWT_SECRET } from "$env/static/private";
import type { Cookies } from "@sveltejs/kit";
import jwt from "jsonwebtoken";
import { A_MONTH_IN_SECONDS } from "./constants";

export interface SignUserOptions {
  id: number;
  cookies: Cookies;
}

export const saveUser = ({ id, cookies }: SignUserOptions) => {
  const token = jwt.sign({ id }, JWT_SECRET, {
    expiresIn: A_MONTH_IN_SECONDS,
  });
  cookies.set("token", token, {
    httpOnly: true,
    maxAge: A_MONTH_IN_SECONDS,
    path: "/",
    sameSite: true,
    secure: true,
  });
};

export interface ClearUserOptions {
  cookies: Cookies;
}

export const clearUser = ({ cookies }: ClearUserOptions) => {
  cookies.delete("token", {
    httpOnly: true,
    maxAge: A_MONTH_IN_SECONDS,
    path: "/",
    sameSite: true,
    secure: true,
  });
};

/**
 * Hash a password using PBKDF2.
 * @param password The raw password.
 * @returns
 */
export const hashPassword = async (password: string) => {
  const enc = new TextEncoder();

  const keyMaterial = await crypto.subtle.importKey("raw", enc.encode(password), "PBKDF2", false, ["deriveBits"]);

  const salt = crypto.getRandomValues(new Uint8Array(16));

  const iterations = 100000;

  const key = await crypto.subtle.deriveBits({ name: "PBKDF2", salt, iterations, hash: "SHA-256" }, keyMaterial, 256);

  const keyHex = Array.from(new Uint8Array(key))
    .map((e) => e.toString(16).padStart(2, "0"))
    .join("");

  const saltHex = Array.from(new Uint8Array(salt))
    .map((e) => e.toString(16).padStart(2, "0"))
    .join("");

  const concat = Buffer.from(`v0$${keyHex}$${saltHex}$${iterations.toString(16)}`);

  return concat.toString("base64");
};

/**
 * Check if an entered password is valid.
 * @param base64Password The actual password.
 * @param checkPassword The password to be checked.
 * @returns
 */
export const verifyPassword = async (base64Password: string, checkPassword: string) => {
  const enc = new TextEncoder();

  const parsedB64Password = Buffer.from(base64Password, "base64").toString().split("$");

  if (parsedB64Password.length !== 4) {
    throw new Error("Invalid password.");
  }

  const [version, keyHex, saltHex, iterationsHex] = parsedB64Password;

  if (version !== "v0") {
    throw new Error("Invalid password.");
  }

  const saltMatches = saltHex.match(/.{2}/g);

  if (!saltMatches) {
    throw new Error("Invalid salt supplied.");
  }

  const salt = new Uint8Array(saltMatches.map((e) => Number.parseInt(e, 16)));

  const iterations = Number.parseInt(iterationsHex, 16);

  const checkKeyMaterial = await crypto.subtle.importKey("raw", enc.encode(checkPassword), { name: "PBKDF2", hash: "SHA-256" }, false, [
    "deriveBits",
  ]);

  const checkKey = await crypto.subtle.deriveBits({ name: "PBKDF2", salt, iterations, hash: "SHA-256" }, checkKeyMaterial, 256);

  const checkKeyHex = Array.from(new Uint8Array(checkKey))
    .map((e) => e.toString(16).padStart(2, "0"))
    .join("");

  return keyHex === checkKeyHex;
};
