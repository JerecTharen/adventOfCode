from lib.Rucksack import Rucksack
from lib.RucksackGroup import RucksackGroup
from lib.PriorityDict import PriorityDict

class PrioritySum:
    def __init__(self, file_str):
        self.rucksacks = []
        self.priority_dict = PriorityDict().get_dict()
        with open(file_str) as file:
            for line in file:
                self.rucksacks.append(Rucksack(line))
    def get_priority_score(self):
        priority_score = 0
        for rucksack in self.rucksacks:
            for common_item in rucksack.common_items:
                priority_score += self.priority_dict[common_item]
        return priority_score
    def get_grouped_priority_score(self):
        priority_score = 0
        current_rucksack_group = []
        for rucksack in self.rucksacks:
            if len(current_rucksack_group) is 2:
                current_rucksack_group.append(rucksack)
                group = RucksackGroup(current_rucksack_group)
                for item in group.get_common_items():
                    priority_score += self.priority_dict[item]
                current_rucksack_group = []
            else:
                current_rucksack_group.append(rucksack)
        return priority_score