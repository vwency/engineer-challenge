import http from "k6/http";
import { headers, BASE_URL } from "../config";

export function loginAndGetSession(
  email = "test@example.com",
  password = "Test1234!",
): Record<string, any> {
  const res = http.post(
    `${BASE_URL}/auth/login`,
    JSON.stringify({ identifier: email, password }),
    { headers },
  );
  return res.cookies;
}
