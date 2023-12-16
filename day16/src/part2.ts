import * as fs from "fs";

type Dir = 0 | 1 | 2 | 3;

type Node = {
  i: number;
  j: number;
  dir: Dir;
};

const input: string[] = fs.readFileSync("./src/input.txt", "utf-8").split("\n");

const getNodeValue = (node: Node) => {
  return input[node.i][node.j];
};

const getNextNodeInDir = (node: Node) => {
  switch (node.dir) {
    case 0:
      if (node.i - 1 >= 0) return { i: node.i - 1, j: node.j, dir: node.dir };
      return null;
    case 1:
      if (node.j + 1 < input[0].length)
        return { i: node.i, j: node.j + 1, dir: node.dir };
      return null;
    case 2:
      if (node.i + 1 < input.length)
        return { i: node.i + 1, j: node.j, dir: node.dir };
      return null;
    case 3:
      if (node.j - 1 >= 0) return { i: node.i, j: node.j - 1, dir: node.dir };
      return null;
    default:
      return null;
  }
};

const pushNextNode = (nodesToExplore: Node[], node: Node) => {
  const nextNode = getNextNodeInDir(node);

  if (nextNode) {
    nodesToExplore.push(nextNode);
  }
  return nodesToExplore;
};

const tracePath = (startingNode: Node) => {
  let nodesToExplore: Node[] = [startingNode];
  let exploredNodes: Record<number, Record<number, Record<Dir, boolean>>> = {};
  input.forEach((row, i) => {
    exploredNodes[i] = {};
    row.split("").forEach((_, j) => {
      exploredNodes[i][j] = {
        0: false,
        1: false,
        2: false,
        3: false,
      };
    });
  });

  while (nodesToExplore.length > 0) {
    // input.forEach((row, i) =>
    //   console.log(
    //     row
    //       .split("")
    //       .map((_, j) =>
    //         exploredNodes[i]?.[j]?.[0] ||
    //         exploredNodes[i]?.[j]?.[1] ||
    //         exploredNodes[i]?.[j]?.[2] ||
    //         exploredNodes[i]?.[j]?.[3]
    //           ? "#"
    //           : input[i][j]
    //       )
    //       .join("")
    //   )
    // );

    // console.log();
    // console.log(nodesToExplore);
    // console.log();

    const node = nodesToExplore.pop();
    if (exploredNodes[node.i]?.[node.j]?.[node.dir]) continue;
    exploredNodes[node.i][node.j][node.dir] = true;

    if (getNodeValue(node) === ".") {
      pushNextNode(nodesToExplore, node);
    }
    if (getNodeValue(node) === "|") {
      if (node.dir === 1 || node.dir === 3) {
        pushNextNode(nodesToExplore, { ...node, dir: 0 });
        pushNextNode(nodesToExplore, { ...node, dir: 2 });
      } else {
        pushNextNode(nodesToExplore, node);
      }
    }
    if (getNodeValue(node) === "-") {
      if (node.dir === 0 || node.dir === 2) {
        pushNextNode(nodesToExplore, { ...node, dir: 1 });
        pushNextNode(nodesToExplore, { ...node, dir: 3 });
      } else {
        pushNextNode(nodesToExplore, node);
      }
    }
    if (getNodeValue(node) === "\\") {
      if (node.dir === 0) {
        pushNextNode(nodesToExplore, { ...node, dir: 3 });
      }
      if (node.dir === 1) {
        pushNextNode(nodesToExplore, { ...node, dir: 2 });
      }
      if (node.dir === 2) {
        pushNextNode(nodesToExplore, { ...node, dir: 1 });
      }
      if (node.dir === 3) {
        pushNextNode(nodesToExplore, { ...node, dir: 0 });
      }
    }

    if (getNodeValue(node) === "/") {
      if (node.dir === 0) {
        pushNextNode(nodesToExplore, { ...node, dir: 1 });
      }
      if (node.dir === 1) {
        pushNextNode(nodesToExplore, { ...node, dir: 0 });
      }
      if (node.dir === 2) {
        pushNextNode(nodesToExplore, { ...node, dir: 3 });
      }
      if (node.dir === 3) {
        pushNextNode(nodesToExplore, { ...node, dir: 2 });
      }
    }
  }

  const visitedNodesCount = Object.values(exploredNodes)
    .map(
      (row) =>
        Object.values(row).filter((col) => col[0] || col[1] || col[2] || col[3])
          .length
    )
    .reduce((a, b) => a + b);
  return visitedNodesCount;
};

let max = 0;
for (let i = 0; i < input.length; i++) {
  const newMax = Math.max(
    tracePath({ i: i, j: 0, dir: 1 }),
    tracePath({ i: i, j: input[0].length - 1, dir: 3 }),
    tracePath({ i: 0, j: i, dir: 2 }),
    tracePath({ i: input.length - 1, j: i, dir: 0 })
  );
  max = Math.max(max, newMax);
}
console.log(max);
