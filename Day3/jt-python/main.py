from lib.PrioritySum import PrioritySum

p_sum = PrioritySum("./ignore_data/input.txt")
print(f"Part 1 sum is {p_sum.get_priority_score()}")
print(f"Part é sum is {p_sum.get_grouped_priority_score()}")