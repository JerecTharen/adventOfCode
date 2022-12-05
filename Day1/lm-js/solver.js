var fs = require("fs");
var text = fs.readFileSync("./input.txt").toString('utf-8');
var textByLine = text.split("\n");

let res = textByLine.reduce((acc, curr) => {
  curr == "" ? acc.push([]) : acc[acc.length - 1].push(curr);
  return acc;
}, [[]]);

const adder = function(x, y){return Number(x) + Number(y)};

let results = []
res.forEach(arr => {
    results.push(arr.reduce(adder, 0))
});

const sorted = results.sort(function(x,y){return y - x});
const first = sorted[0];
const topThree = sorted.slice(0,3);
const sumTopThree = topThree.reduce(adder, 0);

console.log(first)
console.log(sumTopThree)
