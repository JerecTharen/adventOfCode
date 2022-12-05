export enum CardinalDirection {
    north = 'north',
    east = 'east',
    south = 'south',
    west = 'west'
}
export type DirectionNames ={
    north: CardinalDirection,
    east: CardinalDirection,
    south: CardinalDirection,
    west: CardinalDirection
}
export type CoordChange = {
    xChange: 1 | 0 | -1,
    yChange: 1 | 0 | -1,
    newDirection: CardinalDirection
}
export type Direction = {
    name: string,
    l: CoordChange,
    r: CoordChange
}
export type Instruction = {
    facingDirection: 'l' | 'r',
    value: number
}
export type Coordinate = {
    x: number,
    y: number
}