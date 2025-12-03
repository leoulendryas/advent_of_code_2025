import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf8");
const banks = input.trim().split("\n");
const calculate: number[] = [];

for (const line of banks) {
  const bank = line.split("").map(Number);

  let largestNum = -1;
  let firstIdx = -1;
  let secondIdx = -1;

  for (let i = 0; i < bank.length; i++) {
    for (let j = i + 1; j < bank.length; j++) {
      const num = bank[i] * 10 + bank[j];
      if (num > largestNum) {
        largestNum = num;
        firstIdx = i;
        secondIdx = j;
      }
    }
  }

  console.log(largestNum);
  calculate.push(largestNum);
}

const answer = calculate.reduce((x, y) => x + y, 0);
console.log(answer);

