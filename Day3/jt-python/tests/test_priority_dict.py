import unittest

from lib.PriorityDict import PriorityDict

class TestPriorityDict(unittest.TestCase):
    def setUp(self):
        self.priority_dict = PriorityDict()
    def test_dict_shouldHavePriority16_forp(self):
        _expected_priority = 16
        _stub_char = 'p'

        actual_priority_dict = self.priority_dict.get_dict()
        actual_priority = actual_priority_dict[_stub_char]

        self.assertEqual(actual_priority, _expected_priority)
    def test_dict_shouldHavePriority38_forL(self):
        _expected_priority = 38
        _stub_char = 'L'

        actual_priority_dict = self.priority_dict.get_dict()
        actual_priority = actual_priority_dict[_stub_char]

        self.assertEqual(actual_priority, _expected_priority)
    def test_dict_shouldHavePriority42_forP(self):
        _expected_priority = 42
        _stub_char = 'P'

        actual_priority_dict = self.priority_dict.get_dict()
        actual_priority = actual_priority_dict[_stub_char]

        self.assertEqual(actual_priority, _expected_priority)
    def test_dict_shouldHavePriority22_forv(self):
        _expected_priority = 22
        _stub_char = 'v'

        actual_priority_dict = self.priority_dict.get_dict()
        actual_priority = actual_priority_dict[_stub_char]

        self.assertEqual(actual_priority, _expected_priority)
    def test_dict_shouldHavePriority20_fort(self):
        _expected_priority = 20
        _stub_char = 't'

        actual_priority_dict = self.priority_dict.get_dict()
        actual_priority = actual_priority_dict[_stub_char]

        self.assertEqual(actual_priority, _expected_priority)
    def test_dict_shouldHavePriority19_fors(self):
        _expected_priority = 19
        _stub_char = 's'

        actual_priority_dict = self.priority_dict.get_dict()
        actual_priority = actual_priority_dict[_stub_char]

        self.assertEqual(actual_priority, _expected_priority)