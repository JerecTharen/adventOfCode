import { getDirection } from "./getDirection";
import { Coordinate, Direction, Instruction } from "./types";

  export const changeCoord = (instruction: Instruction, currentFacingDirection: Direction, currentCoord: Coordinate): [Direction, Coordinate]=>{
    for(let i = 0; i < Number(instruction.value); i++){
        currentCoord.x += currentFacingDirection[instruction.facingDirection as 'l' | 'r'].xChange;
        currentCoord.y += currentFacingDirection[instruction.facingDirection as 'l' | 'r'].yChange;
    }
    return [getDirection(currentFacingDirection[instruction.facingDirection].newDirection), currentCoord];
  }