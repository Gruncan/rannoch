import os
import re
import sys
from pathlib import Path


class Alt:

    def __init__(self, primary, *alts):
        self.primary = primary
        self.primary_o = "-" + primary
        self.alts = alts
        self.alts_o = ["--" + alt for alt in alts]

    def __contains__(self, item):
        return item == self.primary_o or item in self.alts_o


class JuntPath(Path):

    def __add__(self, other):
        # Not sure internal variable.. this is easier
        if isinstance(other, str):
            return JuntPath(os.path.join(str(self), other))
        elif isinstance(other, Path):
            return JuntPath(os.path.join(str(self), str(other)))
        else:
            # Rust makes me think about other cases!
            raise ValueError(f"Unknown how to add to path type {type(other)}!")


def find_most_recent_file_dir(path: JuntPath, regex=None):
    dir_contents = os.listdir(path)

    last_modified = path + dir_contents[0]
    last_modified_time = os.path.getmtime(last_modified)

    for name in dir_contents[1:]:
        if regex is None or regex.match(name):
            abs_name = path + name
            modified_time = os.path.getmtime(abs_name)

            if modified_time > last_modified_time:
                last_modified_time = modified_time
                last_modified = abs_name

    return last_modified


def main(**kwargs):
    sda_p = JuntPath("/dev/sda")
    sda1_p = JuntPath("/dev/sda1")
    sad2_p = JuntPath("/dev/sda2")

    if not (sda_p.exists() and sda1_p.exists() and sad2_p.exists()):
        raise FileNotFoundError("Failed to parse drive..")

    os.system("sudo umount /dev/sda1")
    os.system("sudo umount /dev/sda2")

    print("Unmounted SDAs...")

    images_p = JuntPath("build/tmp/deploy/images/")

    most_recent_dir = find_most_recent_file_dir(images_p)

    find_most_recent_file_dir(most_recent_dir, re.compile(r".*\.wic\.bz2$"))


def print_help():
    pass


def sda_card():
    pass


argument_map = {
    Alt("h", "help"): print_help,
    Alt("sd", "sda-card"): sda_card,

}


def parse_arguements(args):
    dictionary = {}
    for index, (key, value) in enumerate(argument_map.items()):
        if key in args:
            value()

    return dictionary


if __name__ == '__main__':
    args = sys.argv[1:]
    kwargs = parse_arguements(args)
    main(**kwargs)
