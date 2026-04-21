import { sleep } from "k6";
import { defaultThresholds } from "./config";
import { register } from "./scenarios/register";
import { login } from "./scenarios/login";
import { getCurrentUser } from "./scenarios/me";
import { recovery } from "./scenarios/recovery";

export const options: any = {
  thresholds: defaultThresholds,
  scenarios: {
    max_rps: {
      executor: "ramping-arrival-rate",
      startRate: 10,
      timeUnit: "1s",
      preAllocatedVUs: 50,
      maxVUs: 300,
      stages: [
        { target: 50, duration: "30s" },
        { target: 150, duration: "60s" },
        { target: 300, duration: "60s" },
        { target: 500, duration: "60s" },
        { target: 0, duration: "20s" },
      ],
    },
  },
};

export default function (): void {
  const roll = Math.random();
  if (roll < 0.55) {
    const cookies = login();
    getCurrentUser(cookies);
  } else if (roll < 0.75) {
    register();
  } else {
    recovery();
  }
  sleep(0.1);
}
