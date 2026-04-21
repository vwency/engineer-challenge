import http from "k6/http";
import { check } from "k6";
import { headers, BASE_URL } from "../config";

export function register(): void {
  const uid = `u_${Date.now()}_${Math.random().toString(36).slice(2, 7)}`;
  const res = http.post(
    `${BASE_URL}/auth/register`,
    JSON.stringify({
      identifier: `${uid}@test2.com`,
      username: uid,
      password: "Test1234!",
      geo_location: "ES",
    }),
    { headers },
  );
  if (res.status === 500) {
    console.error(`🔥 500 register URL: ${res.url} body: ${res.body}`);
  }
  check(res, {
    "register 2xx": (r) => r.status >= 200 && r.status < 300,
    "register <500ms": (r) => r.timings.duration < 500,
  });
}
