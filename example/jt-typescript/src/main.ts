import { changeCoord } from "./changeCoord";
import { AllDirectionNames } from "./constants";
import { getDirection } from "./getDirection";
import { Coordinate, Instruction } from "./types";



export const main = (puzzleInput: string): number => {
  console.log('Day 1: No Time for a Taxicab');
  console.log(`Solving: ${puzzleInput}`);



  let coord: Coordinate = {x: 0, y: 0};
  
  let currentFacingDirection = getDirection(AllDirectionNames.north);

  //Parse input setup
  puzzleInput.split(',')
    .map(i => {
      const iString = i.trim();
      const newInstrcution: Instruction = {
        facingDirection: iString[0].toLowerCase() as ('l' | 'r'),
        value: Number(iString.slice(1, iString.length))
      };
      return newInstrcution;
    })
    .forEach(i =>{
      const [newFacingDirection, newCoord] = changeCoord(i, currentFacingDirection, coord);
      coord = newCoord;
      currentFacingDirection = newFacingDirection;
    });
  console.log('You are: ');
  const blockCount = Math.abs(coord.x) + Math.abs(coord.y)
  console.log(blockCount);
  console.log('Blocks away');
  return blockCount;
}
