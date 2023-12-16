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

const tracePath = () => {
  let nodesToExplore: Node[] = [{ i: 0, j: 0, dir: 1 }];
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
  console.log(visitedNodesCount);
};

tracePath();
