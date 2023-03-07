import Parser from "../lib/Parser";
import Crane from "../lib/Crane";

describe('Crane', ()=>{
    const TEST_INPUT = `    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
`;
    it('should be able to follow instructions and produce the expected result from the example input', ()=>{
        const EXPECTED_TOP_STACKS = 'CMZ';

        let stubParser = new Parser(TEST_INPUT);
        let fakeCrane = new Crane(stubParser);
        fakeCrane.followInstructions();
        let actualTopStacks = fakeCrane.getTopCrates();

        expect(actualTopStacks).toBe(EXPECTED_TOP_STACKS);
    });
    it('should be able to follow instructions and produce the expected result from the example input for part 2', ()=>{
        const EXPECTED_TOP_STACKS = 'MCD';
        const isMover9001 = true;

        let stubParser = new Parser(TEST_INPUT);
        let fakeCrane = new Crane(stubParser, isMover9001);
        fakeCrane.followInstructions();
        let actualTopStacks = fakeCrane.getTopCrates();

        expect(actualTopStacks).toBe(EXPECTED_TOP_STACKS);
    });
});