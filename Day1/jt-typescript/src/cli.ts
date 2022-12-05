#!/usr/bin/env node
import { main } from './main'
import {readFile} from 'fs/promises';

console.log('Reading file . . .');
readFile('./ignore_data/input.txt', {encoding: 'utf-8'})
    .then((file: string) => {
        console.log(`File read, input length is: ${file.length}`);
        const result = main(file);
        console.log('Result is: ', result);

        console.log('Calculating top 3 . . .');
        const secondResult = main(file, 3);
        console.log(`Second result is: ${secondResult}`);
    })
