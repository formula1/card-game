import { LangNode, LangNodeType, ValueNode } from "../../types/ReusedStructs";

export function extractNumber(a: LangNode): number {
  if(a.node_type !== LangNodeType.ValueNode) {
    throw new Error("a node_type is not ValueNode");
  }
  if(a.values.type !== "number"){
    throw new Error("expected value type to be number")
  }
  return Number.parseFloat(a.values.value);
}

export function extractPair(a: LangNode, b: LangNode): [number, number]{
  return [ extractNumber(a), extractNumber(b) ];
}

export function extractArray(nums: Array<LangNode>): Array<number> {
  return nums.map((node) => {
    return extractNumber(node);
  });
}



export function packResult(num: number): ValueNode{
  return {
    node_type: LangNodeType.ValueNode,
    values: {
      type: "number",
      is_float: true,
      value: num.toString()
    }
  }
}