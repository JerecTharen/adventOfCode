import unittest
from lib.RucksackGroup import RucksackGroup
from lib.Rucksack import Rucksack

class TestRucksackGroup(unittest.TestCase):
    def test_example1CommonItem_shouldBer(self):
        _expected_common_items = {"r"}

        stubRucksacks = [Rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), Rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), Rucksack("PmmdzqPrVvPwwTWBwg")]
        stubRucksackGroup = RucksackGroup(stubRucksacks)
        actual_common_items = stubRucksackGroup.get_common_items()

        self.assertEqual(actual_common_items, _expected_common_items)
    def test_example2CommonItem_shouldBeZ(self):
        _expected_common_items = {"Z"}

        stubRucksacks = [Rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), Rucksack("ttgJtRGJQctTZtZT"), Rucksack("CrZsJsPPZsGzwwsLwLmpwMDw")]
        stubRucksackGroup = RucksackGroup(stubRucksacks)
        actual_common_items = stubRucksackGroup.get_common_items()

        self.assertEqual(actual_common_items, _expected_common_items)