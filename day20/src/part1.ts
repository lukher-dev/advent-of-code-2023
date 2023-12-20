import * as fs from "fs";

type Gate = "%" | "&" | "b";

type LogicGate = {
  gate: Gate;
  targets: string[];
  prevPulse: boolean;
  inputMem: Record<string, boolean>;
};

type Pulse = {
  gate: string;
  signal: boolean;
  startGate: string;
};

const input: string[] = fs.readFileSync("./src/input.txt", "utf-8").split("\n");

const mappedInput = input.reduce((acc, line) => {
  const splitLine = line.split(" -> ");
  const targets = splitLine[1].split(", ");

  return {
    ...acc,
    [splitLine[0].slice(1)]: {
      gate: splitLine[0].slice(0, 1) as Gate,
      targets: targets,
      prevPulse: false,
      state: splitLine[0].slice(0, 1) === "&" ? true : false,
      inputMem: {},
    },
  };
}, {} as Record<string, LogicGate>);
console.log(mappedInput);
Object.keys(mappedInput).forEach((key) => {
  mappedInput[key].targets.forEach((target) => {
    if (mappedInput[target]) mappedInput[target].inputMem[key] = false;
  });
});

const propagateSignal = (gates: Record<string, LogicGate>, pulse: Pulse) => {
  if (!gates[pulse.gate]) return [];
  gates[pulse.gate].inputMem[pulse.startGate] = pulse.signal;
  if (gates[pulse.gate].gate === "b") {
    gates[pulse.gate].prevPulse = pulse.signal;
    return gates[pulse.gate].targets.map((target) => ({
      startGate: pulse.gate,
      gate: target,
      signal: gates[pulse.gate].prevPulse,
    }));
  }

  if (gates[pulse.gate].gate === "&") {
    if (
      Object.keys(gates[pulse.gate].inputMem).every(
        (key) => gates[pulse.gate].inputMem[key]
      )
    )
      gates[pulse.gate].prevPulse = false;
    else gates[pulse.gate].prevPulse = true;
    return gates[pulse.gate].targets.map((target) => ({
      startGate: pulse.gate,
      gate: target,
      signal: gates[pulse.gate].prevPulse,
    }));
  }

  if (gates[pulse.gate].gate === "%")
    if (pulse.signal == false) {
      gates[pulse.gate].prevPulse = !gates[pulse.gate].prevPulse;
      return gates[pulse.gate].targets.map((target) => ({
        startGate: pulse.gate,
        gate: target,
        signal: gates[pulse.gate].prevPulse,
      }));
    }

  return [];
};

const pushButton = () => {
  const pulses: Pulse[] = [];
  pulses.push({ gate: "roadcaster", signal: false, startGate: "b" });
  let lowCount = 0;
  let highCount = 0;
  while (pulses.length > 0) {
    const pulse = pulses.shift();
    if (pulse.signal === false) lowCount++;
    else highCount++;
    console.log(pulse);
    if (pulse) {
      const newPulses = propagateSignal(mappedInput, pulse);
      pulses.push(...newPulses);
    }
  }
  return { highCount, lowCount };
};

let lowCount = 0;
let highCount = 0;
for (let i = 0; i < 1000; i++) {
  const { highCount: h, lowCount: l } = pushButton();
  lowCount += l;
  highCount += h;
}

console.log({ highCount, lowCount, result: highCount * lowCount });
