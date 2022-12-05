import { Directions } from "./constants";
import { CardinalDirection, Direction } from "./types";

export const getDirection = (dName: CardinalDirection): Direction => Directions.find(d => d.name === dName)!;//Yup, garunteed to have a value here