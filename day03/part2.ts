
import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf8");
const banks = input.trim().split("\n");
const calculate: bigint[] = [];

for (const line of banks) {
  const bank = line.split("").map(Number);
  const topDigits: number[] = [];
  let start = 0;

  while (topDigits.length < 12) {
    const remaining = 12 - topDigits.length;

    const slice = bank.slice(start, bank.length - remaining + 1);

    let maxDigit = -1;
    let maxIdx = -1;
    for (let i = 0; i < slice.length; i++) {
      if (slice[i] > maxDigit) {
        maxDigit = slice[i];
        maxIdx = i;
      }
    }

    topDigits.push(maxDigit);
    start += maxIdx + 1;
  }

  const combinedNumber = BigInt(topDigits.join(""));
  console.log(combinedNumber.toString());
  calculate.push(combinedNumber);
}

const answer = calculate.reduce((acc, num) => acc + num, 0n);
console.log(answer.toString());

