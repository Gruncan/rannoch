import os
import re
import sys
from pathlib import Path
import time


class Alt:

    def __init__(self, primary, *alts):
        self.primary = primary
        self.primary_o = "-" + primary
        self.alts = alts
        self.alts_o = ["--" + alt for alt in alts]

    def __contains__(self, item):
        return item == self.primary_o or item in self.alts_o


class PathAdder(Path):

    def __add__(self, other):
        # Not sure internal variable.. this is easier
        if isinstance(other, str):
            return PathAdder(os.path.join(str(self), other))
        elif isinstance(other, Path):
            return PathAdder(os.path.join(str(self), str(other)))
        else:
            raise ValueError(f"Unknown how to add to path type {type(other)}!")



class FnWrap:
    pass



def find_most_recent_file_dir(path: PathAdder, regex=None):
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

    return last_modified, last_modified_time


def main(**kwargs):
    sda_p = PathAdder("/dev/sda")
    sda1_p = PathAdder("/dev/sda1")
    sad2_p = PathAdder("/dev/sda2")

    if not (sda_p.exists() and sda1_p.exists() and sad2_p.exists()):
        raise FileNotFoundError("Failed to parse drive..")

    os.system("sudo umount /dev/sda1")
    os.system("sudo umount /dev/sda2")

    print("Unmounted SDAs...")

    images_p = PathAdder("tmp/deploy/images")

    most_recent_dir, _ = find_most_recent_file_dir(images_p)

    print(f"Most recent target system: {most_recent_dir.parts[-1]}")

    for f in [most_recent_dir + "image.wic.bz2", most_recent_dir + "image.wic"]:
        if f.exists():
            os.remove(str(f))

    most_recent_file, most_recent_time = find_most_recent_file_dir(most_recent_dir, re.compile(r".*\.wic\.bz2$"))

    print(f"Found {most_recent_file.parts[-1]} created at {time.ctime(most_recent_time)}...")

    print(f"Uncompressing {most_recent_file.parts[-1]}...")


    new_file_path = most_recent_file.with_name(f'image.wic.bz2')

    os.system(f"cp {most_recent_file} {new_file_path}")

    os.system(f"bunzip2 {new_file_path}")

    print("Flashing...")
    os.system(f"sudo dd if={new_file_path.with_name('image.wic')} of=/dev/sda bs=4M status=progress conv=fsync")

    os.system("sync")

    print("Preparing to eject...")
    os.system("sudo eject /dev/sda")

    print("\nSD card is ready to eject...")


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
    # args = sys.argv[1:]
    # kwargs = parse_arguements(args)
    kwargs = {}
    main(**kwargs)
