#!/usr/bin/env node

import { readFile } from "fs/promises";
import Parser from "./lib/Parser";
import Crane from "./lib/Crane";

console.log('Reading file . . .');
readFile('./ignore_data/input.txt', {encoding: 'utf-8'})
    .then((file: string) => {
        console.log(`File read, input length is: ${file.length}`);
        const parsedFile = new Parser(file);
        const crane = new Crane(parsedFile);
        crane.followInstructions();
        console.log(`Top crates after crane instructions: ${crane.getTopCrates()}`);

        console.log(`Simulating Mover 9001 . . .`);
        const mover9001Parse = new Parser(file);
        const mover9001 = new Crane(mover9001Parse, true);
        mover9001.followInstructions();
        console.log(`Mover 9001 receives top crates of: ${mover9001.getTopCrates()}`);
    })
