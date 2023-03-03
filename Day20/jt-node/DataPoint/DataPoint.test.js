const DataPoint = require('./DataPoint');
describe('DataPoint', ()=>{
    it('should be able to store a value', ()=>{
        const EXPECTED_VALUE = 3;

        //System under test
        const SUT = new DataPoint(EXPECTED_VALUE);

        expect(SUT.value).toBe(EXPECTED_VALUE);
    });
    it('should be able to store a boolean', ()=>{
        const EXPECTED_HAS_MIXED = false;

        //System under test
        const SUT = new DataPoint(3);

        expect(SUT.hasMixed).toBe(EXPECTED_HAS_MIXED);
    });
    it('should complain if no number is provided', ()=>{
        let didComplain = false;
        try {
            new DataPoint();
        } catch (error) {
            didComplain = true;
        }
        expect(didComplain).toBe(true);
    });
});