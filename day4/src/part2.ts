import * as fs from "fs";
const input: string = fs.readFileSync("./src/input.txt", "utf-8");
const parsedInput = input.replace(/ +(?= )/g, "").split("\n");
let cards = Array(parsedInput.length).fill(1);
parsedInput
  .map((line) => {
    const splitLine = line.split("|");
    const temp = splitLine[0].trim().split(":");
    const pickedNumbers = splitLine[1].trim().split(" ");
    const luckyNumbers = temp[1].trim().split(" ");
    const filteredArray = pickedNumbers.filter((value) =>
      luckyNumbers.includes(value)
    );
    return filteredArray.length;
  })
  .forEach((result, index) => {
    for (let i = 1; i <= result; i++) {
      cards[index + i] += cards[index];
    }
  });
const result = cards.reduce((partialSum, a) => partialSum + a, 0);
console.log(result);
