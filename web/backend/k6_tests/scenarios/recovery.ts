import http from "k6/http";
import { check } from "k6";
import { headers, BASE_URL } from "../config";

export function recovery(): void {
  const res = http.post(
    `${BASE_URL}/auth/recovery`,
    JSON.stringify({ email: "test@example.com" }),
    { headers },
  );
  if (res.status === 500) {
    console.error(`🔥 500 recovery URL: ${res.url} body: ${res.body}`);
  }
  check(res, { "recovery 2xx": (r) => r.status < 300 });
}
