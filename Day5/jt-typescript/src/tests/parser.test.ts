import Parser from '../lib/Parser';
import Stack from '../lib/Stack';
const TEST_INPUT = `    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
`;

describe('Parser', ()=>{
    describe('separateCratesAndInstructions', ()=>{
        it('should separate the test input into two sections', ()=>{
            const EXPECTED_PARSE_LENGTH = 2;

            let actualParseLength = new Parser(TEST_INPUT).separateCratesAndInstructions(TEST_INPUT).length;

            expect(actualParseLength).toBe(EXPECTED_PARSE_LENGTH);
        });
    });

    describe('createStackArrays', ()=>{
        it('should separate into 3 stacks', ()=>{
            const EXPECTED_PARSE_LENGTH = 3;

            let actualParseLength = new Parser(TEST_INPUT).stacks.length;

            expect(actualParseLength).toBe(EXPECTED_PARSE_LENGTH);
        });
        it('the second stack should be D, C, M', ()=>{
            const EXPECTED_SECOND_STACK = 'DCM';

            let actualStack = '';
            let secondStack: Stack = new Parser(TEST_INPUT).stacks[1];
            for(let letter = 0; letter < EXPECTED_SECOND_STACK.length; letter++){
                actualStack += secondStack.GrabCrate()!.Char;//Fail hard in test
            }

            expect(actualStack).toBe(EXPECTED_SECOND_STACK);
        });
    });

    describe('getInstructions', ()=>{
        it('should find 4 instructions', ()=>{
            const EXPECTED_PARSE_LENGTH = 4;

            let actualParseLength = new Parser(TEST_INPUT).instructions.length;

            expect(actualParseLength).toBe(EXPECTED_PARSE_LENGTH);
        });
        it('the first instruction should be to move 1 crate', ()=>{
            const EXPECTED_VALUE = 1;

            let firstInstruction = new Parser(TEST_INPUT).instructions[0];
            let actualValue = firstInstruction.MoveQauntity;

            expect(actualValue).toBe(EXPECTED_VALUE);
        });
        it('the first instruction should be to move from stack 2', ()=>{
            const EXPECTED_VALUE = 2;

            let firstInstruction = new Parser(TEST_INPUT).instructions[0];
            let actualValue = firstInstruction?.PickupLocation;

            expect(actualValue).toBe(EXPECTED_VALUE);
        });
        it('the first instruction should be to move to stack 1', ()=>{
            const EXPECTED_VALUE = 1;

            let firstInstruction = new Parser(TEST_INPUT).instructions[0];
            let actualValue = firstInstruction?.PlaceLocation;

            expect(actualValue).toBe(EXPECTED_VALUE);
        });
    });
});