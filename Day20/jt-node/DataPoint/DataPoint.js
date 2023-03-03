module.exports = class DataPoint{
    constructor(num){
        if(num === null || num === undefined){
            throw new Error('Data point requires that a number be provided');
        }
        this.value = num;
        this.hasMixed = false;
    }
};