const DataPoint = require('../DataPoint/DataPoint');

module.exports = class Mixer{
    constructor(dataPoints){
        if(!dataPoints){
            throw new Error('No data points provided to Mixer instance');
        }
        if(dataPoints.filter(dp => dp instanceof DataPoint).length !== dataPoints.length){
            throw new Error('All data points must be of type data point');
        }
        this.dataPoints = dataPoints;
    }

    movePoint(pointIndex, isPositive){
        let insertIndex = pointIndex + (isPositive ? 1 : -1);
        if(insertIndex >= this.dataPoints.length){
            insertIndex = 0;
        }
        else if(insertIndex < 0){
            insertIndex = this.dataPoints.length -1;
        }

        const [point] = this.dataPoints.splice(pointIndex, 1, null);
        if(point.hasMixed){
            throw new Error('Cannot move a point that has already mixed!');
        }
        point.hasMixed = true;
        const [switchPoint] = this.dataPoints.splice(insertIndex,1, null);
        this.dataPoints[insertIndex] = point;
        this.dataPoints[pointIndex] = switchPoint;
    }
};