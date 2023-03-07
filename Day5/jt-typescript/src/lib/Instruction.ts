export default class Instruction{
    public readonly MoveQauntity: number;
    public readonly PickupLocation: number;
    public readonly PlaceLocation: number;
    constructor(moveQauntity: number, pickupLocation: number, placeLocation: number) {
        this.MoveQauntity = moveQauntity;
        this.PickupLocation = pickupLocation;
        this.PlaceLocation = placeLocation;
    }
}