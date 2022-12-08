import unittest
from lib.hello_world import HelloWorld

class TestHelloWorld(unittest.TestCase):
    def test_hello_world_should_return_1(self):
        expected_hello_world = 1

        actual_hello_world = HelloWorld().get_one()
        self.assertEqual(actual_hello_world, expected_hello_world)