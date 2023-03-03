const Mixer = require('./Mixer');
const DataPoint = require('../DataPoint/DataPoint');

describe('Mixer', ()=>{
    it('should be able to store an array of DataPoints', ()=>{
        const EXPECTED_DATA_POINTS = [new DataPoint(1), new DataPoint(1), new DataPoint(1)];

        const SUT = new Mixer(EXPECTED_DATA_POINTS);

        expect(SUT.dataPoints).toBe(EXPECTED_DATA_POINTS);
    });
    it('should complain if no data points array is given during construction', ()=>{
        let didComplain = false;
        try {
            new Mixer();
        } catch (error) {
            didComplain = true;
        }
        expect(didComplain).toBe(true);
    });
    it('should complain if the data in the array given during contruction is not a data point', ()=>{
        let didComplain = false;
        try {
            new Mixer([1,2,3]);
        } catch (error) {
            didComplain = true;
        }
        expect(didComplain).toBe(true);
    });

    describe('movePoint', ()=>{
        it('should exist', ()=>{
            const SUT = new Mixer([]);
            expect(SUT.movePoint).toBeTruthy();
        });
        it('should be able to move a data point one space forward', ()=>{
            const EXPECTED_DATA_POINTS = [2,1,2];
            const stubStartingPoints = [new DataPoint(1), new DataPoint(2), new DataPoint(2)];
            const stubMovementIndex = 0;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, true);
            const actualDataPoints = SUT.dataPoints.map(dp => dp.value);

            expect(actualDataPoints).toEqual(EXPECTED_DATA_POINTS);
        });
        it('should be able to move a data point one space forward no matter its value', ()=>{
            const EXPECTED_DATA_POINTS = [1,2,1];
            const stubStartingPoints = [new DataPoint(2), new DataPoint(1), new DataPoint(1)];
            const stubMovementIndex = 0;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, true);
            const actualDataPoints = SUT.dataPoints.map(dp => dp.value);

            expect(actualDataPoints).toEqual(EXPECTED_DATA_POINTS);
        });
        it('should be able to move a data point one space backward', ()=>{
            const EXPECTED_DATA_POINTS = [2,1,1];
            const stubStartingPoints = [new DataPoint(1), new DataPoint(2), new DataPoint(1)];
            const stubMovementIndex = 1;
            
            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, false);
            const actualDataPoints = SUT.dataPoints.map(dp => dp.value);
            
            expect(actualDataPoints).toEqual(EXPECTED_DATA_POINTS);
        });
        it('should be able to move a data point one space backward no matter its value', ()=>{
            const EXPECTED_DATA_POINTS = [1,2,1];
            const stubStartingPoints = [new DataPoint(2), new DataPoint(1), new DataPoint(1)];
            const stubMovementIndex = 1;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, false);
            const actualDataPoints = SUT.dataPoints.map(dp => dp.value);

            expect(actualDataPoints).toEqual(EXPECTED_DATA_POINTS);
        });
        it('should be able to wrap forward', ()=>{
            const EXPECTED_DATA_POINTS = [2,1,1];
            const stubStartingPoints = [new DataPoint(1), new DataPoint(1), new DataPoint(2)];
            const stubMovementIndex = 2;
            const isPositive = true;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, isPositive);
            const actualDataPoints = SUT.dataPoints.map(dp => dp.value);

            expect(actualDataPoints).toEqual(EXPECTED_DATA_POINTS);
        });
        it('should be able to wrap backwards', ()=>{
            const EXPECTED_DATA_POINTS = [1,1,2];
            const stubStartingPoints = [new DataPoint(2), new DataPoint(1), new DataPoint(1)];
            const stubMovementIndex = 0;
            const isPositive = false;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, isPositive);
            const actualDataPoints = SUT.dataPoints.map(dp => dp.value);

            expect(actualDataPoints).toEqual(EXPECTED_DATA_POINTS);
        });
        it('should mark the data point it mixes', ()=>{
            const EXPECTED_HAS_MIXED = true;
            const stubStartingPoints = [new DataPoint(1), new DataPoint(2), new DataPoint(1)];
            const stubMovementIndex = 0;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, true);
            const actualHasMixed = SUT.dataPoints[stubMovementIndex + 1].hasMixed;

            expect(actualHasMixed).toBe(EXPECTED_HAS_MIXED);
        });
        it('should not mark the data point it doesn\'t mix', ()=>{
            const EXPECTED_HAS_MIXED = false;
            const stubStartingPoints = [new DataPoint(1), new DataPoint(2), new DataPoint(1)];
            const stubMovementIndex = 0;

            const SUT = new Mixer(stubStartingPoints);
            SUT.movePoint(stubMovementIndex, true);
            const actualHasMixed = SUT.dataPoints[stubMovementIndex].hasMixed;

            expect(actualHasMixed).toBe(EXPECTED_HAS_MIXED);
        });
        it('should throw if point has already mixed', ()=>{
            let hasError = false;
            const fakePoint = new DataPoint(1);
            fakePoint.hasMixed = true;
            const stubStartingPoints = [fakePoint, new DataPoint(2)];
            const stubMovementIndex = 0;
            const SUT = new Mixer(stubStartingPoints);

            try {
                SUT.movePoint(stubMovementIndex, true);
            } catch (error) {
                hasError = true;
            }
            expect(hasError).toBe(true);
        });
    });

    describe('mix', ()=>{
        it('should exist', ()=>{
            expect(new Mixer([new DataPoint(0)]).mix).toBeTruthy();
        });
    });
});