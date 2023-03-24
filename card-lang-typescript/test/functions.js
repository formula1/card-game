const tap = require("tap");
const assert = require("node:assert");
const { run } = require("../dist");

tap.test("functions", (top)=>{
  function testOp(parent, label, input, expected){
    return parent.test(label, (t)=>{
      var output = run(input);
      t.equal(expected, output);
      t.end()
    });
  }
  top.test("operations", (t)=>{
    testOp(t, "log", `log(8, 2)`, `${Math.log(8)/Math.log(2)}`);
    testOp(t, "log", `log(9, 3)`, `${Math.log(9)/Math.log(3)}`);
    t.end();
  })

  top.test("utility", (t)=>{
    testOp(t, "abs", `abs(-9)`, `${Math.abs(-9)}`);
    testOp(t, "abs", `abs(9)`, `${Math.abs(9)}`);

    testOp(t, "min", `min(-10, 10)`, `${Math.min(-10, 10)}`);
    testOp(t, "min", `min(10, -10)`, `${Math.min(10, -10)}`);
    testOp(t, "min", `min(2, 10)`, `${Math.min(2, 10)}`);
    testOp(t, "min", `min(10, 2)`, `${Math.min(10, 2)}`);
    testOp(t, "min", `min(-2, -10)`, `${Math.min(-2, -10)}`);
    testOp(t, "min", `min(-10, -2)`, `${Math.min(-10, -2)}`);

    testOp(t, "max", `max(-10, 10)`, `${Math.max(-10, 10)}`);
    testOp(t, "max", `max(10, -10)`, `${Math.max(10, -10)}`);
    testOp(t, "max", `max(2, 10)`, `${Math.max(2, 10)}`);
    testOp(t, "max", `max(10, 2)`, `${Math.max(10, 2)}`);
    testOp(t, "max", `max(-2, -10)`, `${Math.max(-2, -10)}`);
    testOp(t, "max", `max(-10, -2)`, `${Math.max(-10, -2)}`);
    t.end();
  })

  top.test("trig", (t)=>{
    testOp(t, "sin-full", "sin(tau)", `${Math.sin(Math.PI * 2)}`);
    testOp(t, "sin-half", "sin(tau/2)", `${Math.sin(Math.PI)}`);
    testOp(t, "sin-pos-quart", "sin(tau/4)", `${Math.sin(Math.PI/2)}`);
    testOp(t, "sin-neg-quart", "sin(-tau/4)", `${Math.sin(-Math.PI/2)}`);

    testOp(t, "cos-full", "cos(tau)", `${Math.cos(Math.PI * 2)}`);
    testOp(t, "cos-half", "cos(tau/2)", `${Math.cos(Math.PI)}`);
    testOp(t, "cos-pos-quart", "cos(tau/4)", `${Math.cos(Math.PI/2)}`);
    testOp(t, "cos-neg-quart", "cos(-tau/4)", `${Math.cos(-Math.PI/2)}`);

    testOp(t, "tan-full", "tan(tau)", `${Math.tan(Math.PI * 2)}`);
    testOp(t, "tan-half", "tan(tau/2)", `${Math.tan(Math.PI)}`);
    testOp(t, "tan-pos-quart", "tan(tau/4)", `${Math.tan(Math.PI/2)}`);
    testOp(t, "tan-neg-quart", "tan(-tau/4)", `${Math.tan(-Math.PI/2)}`);


    testOp(t, "asin-zero", "asin(0)", `${Math.asin(0)}`);
    testOp(t, "asin-pos-quart", "asin(1)", `${Math.asin(1)}`);
    testOp(t, "asin-neg-quart", "asin(-1)", `${Math.asin(-1)}`);

    testOp(t, "acos-zero", "acos(0)", `${Math.acos(0)}`);
    testOp(t, "acos-pos-quart", "acos(1)", `${Math.acos(1)}`);
    testOp(t, "acos-neg-quart", "acos(-1)", `${Math.acos(-1)}`);


    testOp(t, "atan-zero", "atan(0)", `${Math.atan(0)}`);
    testOp(t, "atan-pos-quart", "atan(1)", `${Math.atan(1)}`);
    testOp(t, "atan-neg-quart", "atan(-1)", `${Math.atan(-1)}`);

    t.end();
  });

  top.test("rounding", (t)=>{
    testOp(t, "round", "round(-0.75)", `${Math.round(-0.75)}`);
    testOp(t, "round", "round(-0.5)", `${Math.round(-0.5)}`);
    testOp(t, "round", "round(-0.25)", `${Math.round(-0.25)}`);
    testOp(t, "round", "round(0.25)", `${Math.round(0.25)}`);
    testOp(t, "round", "round(0.5)", `${Math.round(0.5)}`);
    testOp(t, "round", "round(0.75)", `${Math.round(0.75)}`);

    testOp(t, "ceil", "ceil(-0.75)", `${Math.ceil(-0.75)}`);
    testOp(t, "ceil", "ceil(-0.5)", `${Math.ceil(-0.5)}`);
    testOp(t, "ceil", "ceil(-0.25)", `${Math.ceil(-0.25)}`);
    testOp(t, "ceil", "ceil(0.25)", `${Math.ceil(0.25)}`);
    testOp(t, "ceil", "ceil(0.5)", `${Math.ceil(0.5)}`);
    testOp(t, "ceil", "ceil(0.75)", `${Math.ceil(0.75)}`);

    testOp(t, "floor", "floor(-0.75)", `${Math.floor(-0.75)}`);
    testOp(t, "floor", "floor(-0.5)", `${Math.floor(-0.5)}`);
    testOp(t, "floor", "floor(-0.25)", `${Math.floor(-0.25)}`);
    testOp(t, "floor", "floor(0.25)", `${Math.floor(0.25)}`);
    testOp(t, "floor", "floor(0.5)", `${Math.floor(0.5)}`);
    testOp(t, "floor", "floor(0.75)", `${Math.floor(0.75)}`);
    t.end();
  })
  top.end()
})