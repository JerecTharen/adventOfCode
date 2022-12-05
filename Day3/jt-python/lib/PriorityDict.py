class PriorityDict:
    def __init__(self, is_logging_on = False):
        self.priority_dict = {}
        for priority in range(52):
            #Setup lower case vs upper case situational parameters
            is_lower_case_priority = priority < 26
            character_start = 'a' if is_lower_case_priority else 'A'
            priority_modifier = 0 if is_lower_case_priority else 26

            #Add to priority dictionary
            new_char = chr(ord(character_start) + (priority - priority_modifier))
            if(is_logging_on):
                print(f"Adding character {new_char} Priority {priority + 1}")
            self.priority_dict[new_char] = priority + 1
    def get_dict(self):
        return self.priority_dict