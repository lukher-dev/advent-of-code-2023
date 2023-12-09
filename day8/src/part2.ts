import * as fs from "fs";

const input: string = fs
  .readFileSync("./src/input.txt", "utf-8")
  .replace(/[()]/g, "");
const parsedInput = input.replace(/ +(?= )/g, "").split("\n");
const instructions = parsedInput[0].split("") as ("L" | "R")[];

let startNodes: string[] = [];
let endNodes: string[] = [];
const nodeMap = parsedInput.slice(2).reduce(
  (acc, line) => {
    const splitLine = line.split(" = ");
    const node = splitLine[0];
    const [L, R] = splitLine[1].split(", ");

    if (node.endsWith("A")) startNodes = [...startNodes, node];
    if (node.endsWith("Z")) endNodes = [...endNodes, node];

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

let stepCount = 0;
let currentNodes = startNodes;
let cycleDetection = Array(startNodes.length).fill({
  stepsFromStart: null,
  cycleLength: null,
  currentCommand: null,
});
console.log(currentNodes, endNodes);
while (true) {
  currentNodes.forEach((node, index) => {
    if (endNodes.includes(node)) {
      if (
        cycleDetection[index].stepsFromStart !== null &&
        cycleDetection[index].cycleLength === null &&
        cycleDetection[index].currentCommand === stepCount % instructions.length
      ) {
        cycleDetection[index] = {
          ...cycleDetection[index],
          cycleLength: stepCount - cycleDetection[index].stepsFromStart,
        };
      }
      if (cycleDetection[index].stepsFromStart === null) {
        cycleDetection[index] = {
          ...cycleDetection[index],
          stepsFromStart: stepCount,
          currentCommand: stepCount % instructions.length,
        };
      }
    }
  });
  console.log(cycleDetection);
  if (
    cycleDetection.every(
      (v) =>
        v.stepsFromStart !== null &&
        v.cycleLength !== null &&
        v.currentCommand !== null
    )
  )
    break;
  const command = instructions[stepCount % instructions.length];
  currentNodes = currentNodes.map((node) => nodeMap[node][command]);
  stepCount++;
}
