import * as fs from "fs";

const startNode = "AAA";
const endNode = "ZZZ";

const input: string = fs
  .readFileSync("./src/input.txt", "utf-8")
  .replace(/[()]/g, "");
const parsedInput = input.replace(/ +(?= )/g, "").split("\n");
const instructions = parsedInput[0].split("") as ("L" | "R")[];
const nodeMap = parsedInput.slice(2).reduce(
  (acc, line) => {
    const splitLine = line.split(" = ");
    const node = splitLine[0];
    const [L, R] = splitLine[1].split(", ");

    return {
      ...acc,
      [node]: {
        L,
        R,
      },
    };
  },
  {} as {
    [key: string]: { L: string; R: string };
  }
);
console.log(nodeMap);

let stepCount = 0;
let currentNode = startNode;
while (true) {
  if (currentNode === endNode) break;
  const command = instructions[stepCount % instructions.length];
  currentNode = nodeMap[currentNode][command];
  stepCount++;
}

console.log(stepCount);
