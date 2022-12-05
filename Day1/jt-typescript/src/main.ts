import Grouping from './grouping';
import GroupingList from './groupingList';

export const main = (input: string, topCount: number | undefined = undefined): number => {
  //Split each grouping
  const list = new GroupingList(input);
  list.parseInput();
  const calorieSums = list.groupInputs.map(g => {
    //Split each line in the grouping
    const grouping = new Grouping(g);
    grouping.parse();
    return grouping.Sum;
  })

  if(topCount !== undefined){
    if(topCount > calorieSums.length){
      throw new Error(`Top Count ${topCount} is higher than calorie list of length: ${calorieSums.length}`);
    }
    return calorieSums
      //Sort desc
      .sort((a, b) => a > b ? -1 : 1)
      //End is zero based, subtract 1 from length for correct index
      .slice(0, topCount)
      .reduce((sum, elfCalorieTotal) => sum + elfCalorieTotal, 0);
  }


  return Math.max(...calorieSums);
}
