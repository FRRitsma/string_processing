# Intro:
A library to help pre-processing webscraped text files. 

# Functions:
Currently has only one function:

`def filter_list_of_strings(strings: list[str], min_size: int) -> list[str]:`

When scraping webpages a lot of useless text is included: menus, headers, footers. These are large sections of text that are repeated exactly between multiple files. This function detects repeated sections of text between files and removes them.

