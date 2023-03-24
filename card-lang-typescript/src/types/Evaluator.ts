import { Token, LangNode, LangNodeType, ValueNode } from "./ReusedStructs";

export class Evaluator {
  constants: { [key: string]: Constant } = {};

  variables: { [key: string]: Variable } = {};
  arguments: { [key: string]: Variable } = {};

  prefixes:  { [key: string]: PrefixOperator } = {};
  operators: { [key: string]: Operator } = {};
  functions: { [key: string]: Funk } = {};
  constructor(
    constants: Array<Constant>,
    prefixes:  Array<PrefixOperator>,
    operators: Array<Operator>,
    functions: Array<Funk>,
  ){
    for(let value of constants){
      this.constants[value.id] = value;
    }
    for(let value of prefixes){
      this.prefixes[value.id] = value;
    }
    for(let value of operators){
      this.operators[value.id] = value;
    }
    for(let value of functions){
      this.functions[value.id] = value;
    }
  }

  evaluate(parseTree: Array<LangNode>): string{
    return parseTree.map((node)=>{
      return Evaluator.nodeToString(this.parseNode(node))
    }).join("\n")
  }

  private parseNode(node: LangNode): ValueNode {
    switch(node.node_type){
      case LangNodeType.ValueNode: {
        return node
      }
      case LangNodeType.OperatorNode: {
        let op_name = node.values?.type;
        if(op_name == void 0){
          throw new Error("operator node has op type");
        }

        let branches = node.branches;
        if(branches == void 0){
          throw new Error("operator node has no branches");
        }
        let left_branch = branches["left"];
        let right_branch = branches["right"];
        if(right_branch == void 0){
          throw new Error("operator node has no right branch");
        }
        let right = this.parseNode(right_branch);
        if(typeof left_branch == "undefined"){
          return this.prefixes[op_name].runner.run(right)
        } else {
          let left = this.parseNode(left_branch);
          return this.operators[op_name].runner.run(left, right);
        }
      }
      case LangNodeType.IdentifierNode: {
        let name = node.values?.name;
        if(name == void 0){
          throw new Error("identifier node has no name");
        }
        let args = this.arguments;
        let constants = this.constants;
        let variables = this.variables;
        if(name in args){
          return args[name].value;
        }
        if(name in constants){
          return constants[name].value;
        }
        if(name in variables){
          return variables[name].value;
        }
        throw new Error(`Identifier ${name} doesn't exist`);
      }
      case LangNodeType.AssignNode: {
        let name = node.values?.name;
        if(name == void 0){
          throw new Error("identifier node has no name");
        }
        let right = node.branches?.right;
        if(right == void 0){
          throw new Error("identifier node has no values");
        }

        let value = this.parseNode(right);
        this.variables[name] = { id: name, value: value };
        return value;
      }
      case LangNodeType.CallNode: {
        let vals: Array<LangNode> = []
        let args = node.args;
        if(args == void 0){
          throw new Error("no args in callnode");
        }
        let name = node.values?.name;
        if(name == void 0){
          throw new Error("no name in callnode");
        }
        for(let arg of args){
          vals.push(this.parseNode(arg));
        }
        let funk = this.functions[name];
        return funk.runner.run(vals);
      }
      case LangNodeType.FunctionNode: {
        throw new Error("No functions yet");
        /*
        let name = node.values?.name;
        if(name == void 0){
          throw new Error("Function node has no name");
        }

        let names = node.args?.map((arg)=>{
          return arg.values?.name
        })
        if(names == void 0){
          throw new Error("Function node has no args");
        }

        let fn = {
          id: name,
          numArgs: names.length,
          runner: {
            evaluator: this,
            arg_names: names,
            node: node
          }
        };

        this.functions[name] = fn;
        */
      }
      default: {
        throw new Error(`invalid node_type`);
      }
    }
  }

  static nodeToString(node: LangNode): string {
    if(node.node_type != LangNodeType.ValueNode){
      throw new Error("node_type is not ValueNode");
    }
    let values = node.values;
    if(values == void 0){
      throw new Error("node has no values");
    }
    switch(values.type){
      case "boolean": {
        if(values.value == "1"){
          return "True";
        } else {
          return "False";
        }
      }
      case "number": {
        if(values.is_float){
          return Number.parseFloat(values.value).toString()
        } else {
          return Number.parseInt(values.value).toString()
        }
      }
      case "string": {
        return `"${values.value}"`;
      }
      case "function": {
        return "function"
      }
      default: {
        throw new Error("Cannot handle type");
      }
    }
  }
/*
  static valueToString(val: Value) -> String{
    match val {
      Value::BooleanValue(v) => {
        return if v { "True".to_string() } else { "False".to_string() };
      }
      Value::NumberValue(v) => {
        return v.to_string();
      }
      Value::StringValue(v) => {
        return v;
      }
      _ => {
        panic!("invalid value typ")
      }
    }
  }
*/
}

/*
pub enum Value {
  BooleanValue(bool),
  NumberValue(f64),
  StringValue(String),
  FunkValue
}
*/

export type Constant = {
  id: string,
  value: ValueNode
}

type Variable = {
  id: string,
  value: ValueNode
}

export type PrefixOperator = {
  id: string,
  runner: PrefixRunner
}

export interface PrefixRunner {
  run(a: LangNode): ValueNode;
}

export type Operator = {
  id: string,
  runner: OperatorRunner
}

export interface OperatorRunner {
  run(a: LangNode, b: LangNode): ValueNode;
}

export type Funk = {
  id: string,
  numArgs: number,
  runner: FunkRunner
}

export interface FunkRunner {
  run(args: Array<LangNode> ): ValueNode;
}

type CreatedFunk = {
  evaluator: Evaluator,
  arg_names: Array<string>,
  node: LangNode
}

// impl FunkRunner for CreatedFunk {
//   fn run(self, args: Vec<Node>) -> Node {
//     for (i, arg_name) in self.arg_names.iter().enumerate() {
//       self.evaluator.arguments.insert(
//         *arg_name, Variable{ id: *arg_name, value: args[i] }
//       );
//     }
//     let ret = self.evaluator.parseNode(*self.node.branches.unwrap().get("block").unwrap());
//     self.evaluator.arguments.clear();
//     return ret;
//   }
// }