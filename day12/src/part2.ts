import * as fs from "fs";

export function memoize<Args extends unknown[], Result>(
  func: (...args: Args) => Result
): (...args: Args) => Result {
  const stored = new Map<string, Result>();

  return (...args) => {
    const k = JSON.stringify(args);
    if (stored.has(k)) {
      return stored.get(k)!;
    }
    const result = func(...args);
    stored.set(k, result);
    return result;
  };
}

type MappedInput = { expected: number[]; positions: string[] };

const input: string[] = fs.readFileSync("./src/input.txt", "utf-8").split("\n");
const mappedInput: MappedInput[] = input.map((line) => {
  const splitLine = line.split(" ");
  const expected = Array(5)
    .fill(splitLine[1])
    .join(",")
    .split(",")
    .map((val) => Number(val));

  const positions = Array(5)
    .fill(splitLine[0].replace(/\.+/g, "."))
    .join("?")
    .split("");
  return { expected, positions };
});

const resolve = memoize((data: MappedInput) => {
  if (data.expected.length === 0) {
    if ((data.positions.join("").match(/#/g) || []).length > 0) return 0;
    return 1;
  }

  if (
    (data.positions.join("").match(/#|\?/g) || []).length <
    data.expected.reduce(
      (accumulator, currentValue) => accumulator + currentValue,
      0
    )
  )
    return 0;

  if (data.positions[0] === ".") {
    return resolve({
      positions: data.positions.slice(1),
      expected: data.expected,
    });
  }

  if (data.positions[0] === "#") {
    if (
      (data.positions.slice(0, data.expected[0]).join("").match(/\./g) || [])
        .length === 0 &&
      (data.positions[data.expected[0]] === "?" ||
        data.positions[data.expected[0]] === "." ||
        !data.positions[data.expected[0]])
    )
      return resolve({
        positions: [...data.positions.slice(data.expected[0] + 1)],
        expected: data.expected.slice(1),
      });
    return 0;
  }

  return (
    resolve({
      positions: [".", ...data.positions.slice(1)],
      expected: data.expected,
    }) +
    resolve({
      positions: ["#", ...data.positions.slice(1)],
      expected: data.expected,
    })
  );
});

console.log(
  mappedInput.reduce((acc, input) => {
    return acc + resolve(input);
  }, 0)
);
