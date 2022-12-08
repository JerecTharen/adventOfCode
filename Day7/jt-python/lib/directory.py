from lib.file import File
from typing import Union
class Directory:
    def __init__(self, name, contents: list):
        self._contents = contents
        self._name = name
    def get_contents(self):
        return self._contents
    def get_child_files(self):
        return list(filter(lambda content : content.__class__ == File, self._contents))
    def get_child_directories(self):
        return list(filter(lambda content : content.__class__ == Directory, self._contents))
    def get_name(self):
        return self._name