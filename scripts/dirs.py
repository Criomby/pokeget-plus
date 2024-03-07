"""
Script to generate items categories and lists from the pokesprite database.
"""

import os
import json


DATA_DIR = "../data/"
POKESPRITE_DIR = "../data/pokesprite/"


def get_subdirs(dir: str) -> list[str, ...]:
    """
    Get subdirs from specified dir.
    Returns the list of subdirs.
    """
    return [f.name for f in os.scandir(dir) if f.is_dir()]


def get_items_list(filepath = POKESPRITE_DIR + "data/item-map.json") -> str:
    """
    Parse list of items provided by pokesprite.
    """
    data = {}

    with open(filepath) as f:
        data = json.load(f)
    
    new_dict = {}
    items = []
    
    for key, item in data.items():
        new_dict[int(key[5:])] = item
        items.append(item)
    
    for item in sorted(items):
        print(item)

    #return json.dumps(new_dict, indent=4)


if __name__ == "__main__":
    get_items_list()
