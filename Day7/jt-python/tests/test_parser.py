import unittest
from lib.directory import Directory
from lib.file import File

from lib.parser import Parser
TEST_INPUT = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""

class TestParser(unittest.TestCase):
    def test_parser_should_parse_dirs(self):
        EXPECTED_DIRECTORY_INPUT = ["/", "a", "e", "d"]

        fakeParser = Parser(TEST_INPUT)
        fakeParser.parse()
        actual_directory_inputs = fakeParser.get_directory_inputs()

        self.assertEqual(actual_directory_inputs, EXPECTED_DIRECTORY_INPUT)

    def test_parser_should_parse_file_names_length_3(self):
        EXPECTED_FILE_INPUTS_LENGTH = 4

        fake_parser = Parser(TEST_INPUT)
        fake_parser.parse()
        actual_file_inputs_length = len(fake_parser.get_file_inputs())

        self.assertEqual(actual_file_inputs_length, EXPECTED_FILE_INPUTS_LENGTH)
    def test_parser_should_parse_file_names(self):
        EXPECTED_FILE_INPUTS = [["14848514 b.txt", "8504156 c.dat"], ["29116 f", "2557 g", "62596 h.lst"], ["584 i"], ["4060174 j", "8033020 d.log", "5626152 d.ext", "7214296 k"]]

        fake_parser = Parser(TEST_INPUT)
        fake_parser.parse()
        actual_file_inputs = fake_parser.get_file_inputs()

        self.assertEqual(actual_file_inputs, EXPECTED_FILE_INPUTS)

    def test_parser_get_parsed_should_return_4_dir(self):
        EXPECTED_LEN = 4

        fake_parser = Parser(TEST_INPUT)
        fake_parser.parse()
        parsed = fake_parser.get_parsed()
        actual_dir_len = len(parsed)

        self.assertEqual(actual_dir_len, EXPECTED_LEN)
    def test_parser_get_parsed_first_dir_should_return_2_files(self):
        EXPECTED_LEN = 2

        fake_parser = Parser(TEST_INPUT)
        fake_parser.parse()
        parsed = fake_parser.get_parsed()
        filtered = parsed[0].get_child_files()
        actual_dir_len = len(list(filtered))

        self.assertEqual(actual_dir_len, EXPECTED_LEN)
    def test_parser_get_parsed_first_dir_first_file_should_be_14848514_size(self):
        EXPECTED_SIZE = 14848514

        fake_parser = Parser(TEST_INPUT)
        fake_parser.parse()
        parsed = fake_parser.get_parsed()
        actual_dir_size = parsed[0].get_child_files()[0].get_size()

        self.assertEqual(actual_dir_size, EXPECTED_SIZE)

    def test_get_below_threshold_sum_returns_95437(self):
        EXPECTED_SUM = 95437

        fake_parser = Parser(TEST_INPUT)
        fake_parser.parse()
        actual_sum = fake_parser.get_below_threshold_sum()

        self.assertEqual(actual_sum, EXPECTED_SUM)
