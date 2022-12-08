from lib.directory import Directory
from lib.file import File
from lib.constants import MAX_SIZE


class Parser:
    def __init__(self, input: str, is_debug_on = False):
        self._input = input
        self._directories = []
        self._files = []
        self._is_debug_on = is_debug_on
        self._parsed = []
    def parse(self):
        #TODO: Handle sub directory situation
        lines = self._input.splitlines()
        next_dirname = ""
        is_in_ls = False
        next_files = []
        for line in lines:
            if line[0] == "$" and is_in_ls:
                is_in_ls = False
                self._files.append(next_files)
                next_files = []
                next_dirname = ""

            if line[0] == "$" and ("cd" in line):
                next_dirname = line.split(" ")[2]
            elif line[0] == "$" and ("ls" in line) and next_dirname != "":
                self._directories.append(next_dirname)
                is_in_ls = True
            elif line[0] != "$" and is_in_ls and line[0:3] != "dir":
                if self._is_debug_on: print(f"Parsing file line {line}")
                next_files.append(line)
            else:
                print("Warning: Cannot parse line - " + line)
        self._files.append(next_files)

    def get_directory_inputs(self):
        if len(self._directories) == 0: raise Exception("Need to parse before getting directory inputs")
        return self._directories
    def get_file_inputs(self):
        if len(self._files) == 0: raise Exception("Need to parse before getting file inputs")
        return self._files

    def get_parsed(self):
        dirs = self.get_directory_inputs()
        files = self.get_file_inputs()
        for idx, dir_name in enumerate(dirs):
            dir_full_files = []
            for raw_file in files[idx]:
                [file_size, file_name] = raw_file.split(" ")
                dir_full_files.append(File(file_name, int(file_size)))
            full_dir = Directory(dir_name, dir_full_files)
            self._parsed.append(full_dir)
        return self._parsed

    def get_below_threshold_sum(self):
        sum = 0
        for dir in self.get_parsed():
            dir_sum = 0
            for file in dir.get_child_files():
                dir_sum += file.get_size()
            print(f"Debug file sum dir {dir.get_name()} is {dir_sum}")
            if dir_sum < MAX_SIZE:
                sum += dir_sum
        return sum