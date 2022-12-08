import unittest
from lib.directory import Directory
from lib.file import File

class TestDirectory(unittest.TestCase):
    def test_get_child_files_returns_files(self):
        EXPECTED_CLASS = File

        fake_directory = Directory("a",[File("fake", 1), Directory("b",[File("fake2", 2)])])
        actual_class = fake_directory.get_child_files()[0].__class__

        self.assertEqual(EXPECTED_CLASS, actual_class)
    def test_get_child_files_returns_direct_child_file(self):
        EXPECTED_NAME = "TEST"

        fake_directory = Directory("a",[File(EXPECTED_NAME, 1), Directory("b",[File("fake2", 2)])])
        actual_name = fake_directory.get_child_files()[0].get_name()

        self.assertEqual(EXPECTED_NAME, actual_name)

    def test_get_child_directories_returns_directories(self):
        EXPECTED_CLASS = Directory

        fake_directory = Directory("a",[File("fake", 1), Directory("b",[File("fake2", 2)])])
        actual_class = fake_directory.get_child_directories()[0].__class__

        self.assertEqual(EXPECTED_CLASS, actual_class)
    def test_get_child_directories_returns_direct_child_directory(self):
        EXPECTED_NAME = "b"

        fake_directory = Directory("a",[File(EXPECTED_NAME, 1), Directory("b",[File("fake2", 2), Directory("c",[])])])
        actual_name = fake_directory.get_child_directories()[0].get_name()

        self.assertEqual(EXPECTED_NAME, actual_name)