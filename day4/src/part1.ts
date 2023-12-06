import * as fs from "fs";
const input: string = fs.readFileSync("./src/input.txt", "utf-8");
const parsedInput = input.replace(/ +(?= )/g, "").split("\n");
const result = parsedInput
  .map((line) => {
    const splitLine = line.split("|");
    const temp = splitLine[0].trim().split(":");
    const pickedNumbers = splitLine[1].trim().split(" ");
    const luckyNumbers = temp[1].trim().split(" ");
    const filteredArray = pickedNumbers.filter((value) =>
      luckyNumbers.includes(value)
    );
    return filteredArray.length > 0 ? 2 ** (filteredArray.length - 1) : 0;
  })
  .reduce((partialSum, a) => partialSum + a, 0);
console.log(result);
