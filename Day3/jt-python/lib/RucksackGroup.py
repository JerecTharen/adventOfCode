class RucksackGroup:
    def __init__(self, rucksack_list):
        self.rucksack_list = rucksack_list
    def get_common_items(self):
        common_items = set([])
        for compartment in self.rucksack_list[0].compartments:
            for item in compartment:
                is_in_second = (item in self.rucksack_list[1].compartments[0]) or (item in self.rucksack_list[1].compartments[1])
                is_in_third = (item in self.rucksack_list[2].compartments[0]) or (item in self.rucksack_list[2].compartments[1])
                if(is_in_second and is_in_third):
                    common_items.add(item)
        return common_items