import { readFileSync } from "fs";

const input = readFileSync("day01/input.txt", "utf8");
const paths = input.trim().split("\n");

let start = 50;
let count = 0;

try {
  for (const path of paths) {
    const direction = path[0];
    const steps = parseInt(path.slice(1), 10);
    let result: number;

    if (direction === "R") {
      result = (start + steps) % 100;
    } else if (direction === "L") {
      result = (start - steps + 100) % 100;
    } else {
      console.log("Unknown direction:", direction);
      continue;
    }

    if (result === 0) {
      count++;
    }

    start = result;
  }

  console.log(count);
} catch (err) {
  console.log("Error:", err);
}
