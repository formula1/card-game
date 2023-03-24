const tap = require("tap");
const assert = require("node:assert");
const { run } = require("../dist");

tap.test("basic operations", (top)=>{
  function testOp(label, input, expected){
    return top.test(label, (t)=>{
      var output = run(input);
      t.equal(expected, output);
      t.end()
    });
  }
  testOp("multiplication before addition", `2 + 1 * 1`, `${2 + 1 * 1}`);
  testOp("multiplication before addition", `1 * 1 + 2`, `${1 * 1 + 2}`);
  testOp("multiplication before subtraction", `2 - 1 * 1`, `${2 - 1 * 1}`);
  testOp("multiplication before subtraction", `1 * 1 - 2`, `${1 * 1 - 2}`);

  testOp("division before addition", `2 + 1 / 2`, `${2 + 1 / 2}`);
  testOp("division before addition", `1 / 2 + 2`, `${1 / 2 + 2}`);
  testOp("division before subtraction", `2 - 1 / 2`, `${2 - 1 / 2}`);
  testOp("division before subtraction", `1 / 2 - 2`, `${1 / 2 - 2}`);

  testOp("modulus before addition", `2 + 3 % 2`, `${2 + 3 % 2}`);
  testOp("modulus before addition", `3 % 2 + 2`, `${3 % 2 + 2}`);
  testOp("modulus before subtraction", `2 - 3 % 2`, `${2 - 3 % 2}`);
  testOp("modulus before subtraction", `3 % 2 - 2`, `${3 % 2 - 2}`);

  testOp("exponent before addition", `2 + 2 ^ 3`, `${2 + Math.pow(2, 3)}`);
  testOp("exponent before addition", `2 ^ 3 + 2`, `${Math.pow(2, 3) + 2}`);
  testOp("exponent before subtraction", `2 - 2 ^ 3`, `${2 - Math.pow(2, 3)}`);
  testOp("exponent before subtraction", `2 ^ 3 - 2`, `${Math.pow(2, 3) - 2}`);

  testOp("exponent before multiplication", `2 * 2 ^ 3`, `${2 * Math.pow(2, 3)}`);
  testOp("exponent before multiplication", `2 ^ 3 * 2`, `${Math.pow(2, 3) * 2}`);
  testOp("exponent before division", `8 / 2 ^ 3`, `${8 / Math.pow(2, 3)}`);
  testOp("exponent before division", `2 ^ 3 / 8`, `${Math.pow(2, 3) / 8}`);
  testOp("modulus before subtraction", `2 ^ 3 % 5`, `${Math.pow(2, 3) % 5}`);
  testOp("modulus before subtraction", `5 % 2 ^ 3`, `${5 % Math.pow(2, 3)}`);

  testOp("parenthesis before exponent", `2 ^ (4/2)`, `${Math.pow(2, 4/2)}`);
  testOp("parenthesis before exponent", `(4/2) ^ 2`, `${Math.pow(4/2, 2)}`);
  testOp("parenthesis before parenthesis", `2^((2*2)/(2+2))`, `${Math.pow(2, ((2*2)/(2+2)))}`);

  testOp("prefix before exponent", `2 ^ -1`, `${Math.pow(2, -1)}`);
  testOp("prefix before exponent", `-1 ^ 2`, `${Math.pow(-1, 2)}`);

  top.end()
})

tap.pass("jest tusting")
