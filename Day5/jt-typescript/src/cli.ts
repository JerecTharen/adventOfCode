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
    })
