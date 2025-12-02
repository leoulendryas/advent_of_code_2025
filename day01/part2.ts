import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf8");
const paths = input.trim().split("\n");

let start = 50;
let count = 0;

for (const path of paths) {
  const direction = path[0];
  const steps = parseInt(path.slice(1), 10);

  for (let i = 0; i < steps; i++) {
    if (direction === "R") {
      start = (start + 1) % 100;
    } else if (direction === "L") {
      start = (start - 1 + 100) % 100;
    } else {
      console.log("Unknown direction:", direction);
    }

    if (start === 0) {
      count++;
    }
  }
}

console.log(count);
