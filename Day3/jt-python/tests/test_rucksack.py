import unittest
from lib.Rucksack import Rucksack

class TestRucksack(unittest.TestCase):
    def test_get_compartments_shouldReturnSplitfor_vJrwpWtwJgWrhcsFMMfFFhFp(self):
        _expected_compartments = ["vJrwpWtwJgWr", "hcsFMMfFFhFp"]
        _stub_input = "vJrwpWtwJgWrhcsFMMfFFhFp"

        rucksack = Rucksack(_stub_input)
        actual_compartments = rucksack._get_compartments(_stub_input)

        self.assertEqual(actual_compartments, _expected_compartments)
    def test_get_compartments_shouldReturnSplitfor_jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL(self):
        _expected_compartments = ["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]
        _stub_input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"

        rucksack = Rucksack(_stub_input)
        actual_compartments = rucksack._get_compartments(_stub_input)

        self.assertEqual(actual_compartments, _expected_compartments)
    def test_get_compartments_shouldReturnSplitfor_PmmdzqPrVvPwwTWBwg(self):
        _expected_compartments = ["PmmdzqPrV", "vPwwTWBwg"]
        _stub_input = "PmmdzqPrVvPwwTWBwg"

        rucksack = Rucksack(_stub_input)
        actual_compartments = rucksack._get_compartments(_stub_input)

        self.assertEqual(actual_compartments, _expected_compartments)
    def test_get_compartments_shouldReturnSplitfor_wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn(self):
        _expected_compartments = ["wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"]
        _stub_input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"

        rucksack = Rucksack(_stub_input)
        actual_compartments = rucksack._get_compartments(_stub_input)

        self.assertEqual(actual_compartments, _expected_compartments)
    def test_get_compartments_shouldReturnSplitfor_ttgJtRGJQctTZtZT(self):
        _expected_compartments = ["ttgJtRGJ", "QctTZtZT"]
        _stub_input = "ttgJtRGJQctTZtZT"

        rucksack = Rucksack(_stub_input)
        actual_compartments = rucksack._get_compartments(_stub_input)

        self.assertEqual(actual_compartments, _expected_compartments)
    def test_get_compartments_shouldReturnSplitfor_CrZsJsPPZsGzwwsLwLmpwMDw(self):
        _expected_compartments = ["CrZsJsPPZsGz", "wwsLwLmpwMDw"]
        _stub_input = "CrZsJsPPZsGzwwsLwLmpwMDw"

        rucksack = Rucksack(_stub_input)
        actual_compartments = rucksack._get_compartments(_stub_input)

        self.assertEqual(actual_compartments, _expected_compartments)

    def test_get_common_items_vJrwpWtwJgWrhcsFMMfFFhFp_shouldBep(self):
        _expected_common_items = {"p"}
        _stub_input = "vJrwpWtwJgWrhcsFMMfFFhFp"

        rucksack = Rucksack(_stub_input)
        actual_common_items = rucksack._get_common_items(rucksack.compartments)

        self.assertEqual(actual_common_items, _expected_common_items)
    def test_get_common_items_jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL_shouldBeL(self):
        _expected_common_items = {"L"}
        _stub_input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"

        rucksack = Rucksack(_stub_input)
        actual_common_items = rucksack._get_common_items(rucksack.compartments)

        self.assertEqual(actual_common_items, _expected_common_items)
    def test_get_common_items_PmmdzqPrVvPwwTWBwg_shouldBeP(self):
        _expected_common_items = {"P"}
        _stub_input = "PmmdzqPrVvPwwTWBwg"

        rucksack = Rucksack(_stub_input)
        actual_common_items = rucksack._get_common_items(rucksack.compartments)

        self.assertEqual(actual_common_items, _expected_common_items)
    def test_get_common_items_wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn_shouldBeP(self):
        _expected_common_items = {"v"}
        _stub_input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"

        rucksack = Rucksack(_stub_input)
        actual_common_items = rucksack._get_common_items(rucksack.compartments)

        self.assertEqual(actual_common_items, _expected_common_items)
    def test_get_common_items_ttgJtRGJQctTZtZT_shouldBet(self):
        _expected_common_items = {"t"}
        _stub_input = "ttgJtRGJQctTZtZT"

        rucksack = Rucksack(_stub_input)
        actual_common_items = rucksack._get_common_items(rucksack.compartments)

        self.assertEqual(actual_common_items, _expected_common_items)
    def test_get_common_items_CrZsJsPPZsGzwwsLwLmpwMDw_shouldBes(self):
        _expected_common_items = {"s"}
        _stub_input = "CrZsJsPPZsGzwwsLwLmpwMDw"

        rucksack = Rucksack(_stub_input)
        actual_common_items = rucksack._get_common_items(rucksack.compartments)

        self.assertEqual(actual_common_items, _expected_common_items)