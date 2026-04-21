export const BASE_URL: string =
  __ENV.BASE_URL || "http://localhost:8080/api/v1";
export const headers: Record<string, string> = {
  "Content-Type": "application/json",
};
export const defaultThresholds = {
  http_req_duration: ["p(95)<500", "p(99)<1000"],
  http_req_failed: ["rate<0.01"],
};
