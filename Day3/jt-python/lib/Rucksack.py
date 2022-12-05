class Rucksack:
    def __init__(self, input_str):
        self.input_str = input_str.strip()
        self.compartments = self._get_compartments(self.input_str)
        self.common_items = self._get_common_items(self.compartments)
    def __str__(self) -> str:
        return str(self.compartments)
    
    def _get_compartments(self, input_str):
        input_length = len(input_str)
        middle_index = int(input_length/2)
        first_str = input_str[0:middle_index]
        second_str = input_str[middle_index: input_length]
        return [first_str, second_str]
    
    def _get_common_items(self, compartments):
        first = compartments[0]
        second = compartments[1]
        common_list = []

        for item in first:
            if(item in second):
                common_list.append(item)

        return set(common_list)