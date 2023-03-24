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
  for(var i = 0; i < 4; i++){
    testOp("addition - " + i, `${i} + ${i}`, `${i + i}`);
  }
  for(var i = 0; i < 4; i++){
    testOp("subtraction - " + i, `${i * 2} - ${i}`, `${(i*2) - i}`);
  }

  for(var i = 0; i < 4; i++){
    testOp("multiplication - " + i, `${i} * ${i}`, `${i * i}`);
  }
  for(var i = 1; i < 5; i++){
    testOp("division - " + i, `${i * 3} / ${i}`, `${(i*3) / i}`);
  }

  for(var i = 1; i < 5; i++){
    testOp("exponent - " + i, `${i} ^ ${i}`, `${Math.pow(i, i)}`);
  }
  for(var i = 1; i < 5; i++){
    testOp("root - " + i, `${i * i} ^ (1/2)`, `${i}`);
  }
  // for(var i = 1; i < 5; i++){
  //   testOp("negative exponent - " + i, `${i * i} ^ (-1)`, `${i}`);
  // }

  top.end()
})

tap.pass("jest tusting")