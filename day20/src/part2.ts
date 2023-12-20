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
  let xj = false;
  let qs = false;
  let kz = false;
  let km = false;
  while (pulses.length > 0) {
    const pulse = pulses.shift();
    if (mappedInput["xj"].prevPulse === true) xj=true;
    if (mappedInput["qs"].prevPulse === true) qs=true;
    if (mappedInput["kz"].prevPulse === true) kz=true;
    if (mappedInput["km"].prevPulse === true) km=true;
    if (pulse.signal === false) lowCount++;
    else highCount++;
    if (pulse) {
      const newPulses = propagateSignal(mappedInput, pulse);
      pulses.push(...newPulses);
    }
  }
  return { highCount, lowCount, xj, qs, kz, km };
};

let lowCount = 0;
let highCount = 0;

for (let i = 0; i < 1000000; i++) {
  const { highCount: h, lowCount: l, xj, qs, kz, km } = pushButton();
  if (xj) console.log({ xj, buttonPresses: i + 1 });
  if (qs) console.log({ qs, buttonPresses: i + 1 });
  if (kz) console.log({ kz, buttonPresses: i + 1 });
  if (km) console.log({ km, buttonPresses: i + 1 });

  lowCount += l;
  highCount += h;
}
