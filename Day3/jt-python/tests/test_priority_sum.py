import unittest
from lib.PrioritySum import PrioritySum

class TestPrioritySum(unittest.TestCase):
    def test_get_priority_score_withExample_shouldBe157(self):
        _expected_score = 157

        priority_sum = PrioritySum("./ignore_data/example.txt")
        actual_score = priority_sum.get_priority_score()

        self.assertEqual(actual_score, _expected_score)
    def test_get_grouped_piority_score_withExample_shouldBe70(self):
        _expected_score = 70

        priority_sum = PrioritySum('./ignore_data/example.txt')
        actual_score = priority_sum.get_grouped_priority_score()
        
        self.assertEqual(actual_score, _expected_score)