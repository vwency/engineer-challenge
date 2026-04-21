import http from "k6/http";
import { check } from "k6";
import { BASE_URL } from "../config";

export function getCurrentUser(cookies: Record<string, any>): void {
  const res = http.get(`${BASE_URL}/auth/me`, { cookies });
  if (res.status === 500) {
    console.error(`🔥 500 me URL: ${res.url} body: ${res.body}`);
  }
  check(res, { "me 200": (r) => r.status === 200 });
}
