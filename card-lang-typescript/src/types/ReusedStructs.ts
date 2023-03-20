
export type Token = {
  token_type: string,
  values: {[key: string]: string }
}

export enum LangNodeType {
  ValueNode = "value",
  OperatorNode = "operator",
  IdentifierNode = "identifier",
  AssignNode = "assign",
  CallNode = "call",
  FunctionNode = "function"
}

export type LangNode = (
  | ValueNode
  | OperatorNode
  | IdentifierNode
  | AssignNode
  | CallNode
  | FunctionNode
);

/*

export type LangNode = {
  node_type: LangNodeType,
} & Partial<{
  values: {[key: string]: string }
  branches: {[key: string]: LangNode }
  args: Array<LangNode>
}>

*/

export type ValueNode = {
  node_type: LangNodeType.ValueNode,
  values: BoolanValue | NumberValue | StringValue | FunctionValue
}

type BoolanValue = {
  type: "boolean",
  value: "1" | "0"
}

type NumberValue = {
  type: "number"
  is_float: boolean,
  value: string
}

type StringValue = {
  type: "string"
  value: string
}

type FunctionValue = {
  type: "function"
}


export type OperatorNode = {
  node_type: LangNodeType.OperatorNode,
  values: { type: string },
  branches: {
    left?: void | LangNode
    right: LangNode
  }
}


export type IdentifierNode = {
  node_type: LangNodeType.IdentifierNode,
  values: { name: string }
}


export type AssignNode = {
  node_type: LangNodeType.AssignNode,
  values: { name: string },
  branches: {
    right: LangNode
  }
}


export type CallNode = {
  node_type: LangNodeType.CallNode,
  values: { name: string },
  args: Array<LangNode>
}


export type FunctionNode = {
  node_type: LangNodeType.FunctionNode,
  values: { name: string },
  args: Array<IdentifierNode>,
  branches: {
    right: LangNode
  }
}