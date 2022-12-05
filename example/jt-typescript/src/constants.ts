import { DirectionNames, CardinalDirection, Direction } from "./types"

  //Object oriented direction logic setup
export const AllDirectionNames: DirectionNames = {
    north: CardinalDirection.north,
    east: CardinalDirection.east,
    south: CardinalDirection.south,
    west: CardinalDirection.west,
}
export const Directions: Direction[] = [
  {name: AllDirectionNames.north, l: {xChange: -1, yChange: 0, newDirection: AllDirectionNames.west}, r: {xChange: 1, yChange: 0, newDirection: AllDirectionNames.east},},
  {name: AllDirectionNames.east, l: {xChange: 0, yChange: 1, newDirection: AllDirectionNames.north}, r: {xChange: 0, yChange: -1, newDirection: AllDirectionNames.south}},
  {name: AllDirectionNames.south, l: {xChange: 1, yChange: 0, newDirection: AllDirectionNames.east}, r: {xChange: -1, yChange: 0, newDirection: AllDirectionNames.west}},
  {name: AllDirectionNames.west, l: {xChange: 0, yChange: -1, newDirection: AllDirectionNames.south}, r: {xChange: 0, yChange: 1, newDirection: AllDirectionNames.north}},
]