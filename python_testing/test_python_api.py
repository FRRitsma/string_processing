from string_processing import filter_list_of_strings

# print(string_processing.__all__)
help(filter_list_of_strings)

# from string_processing.string_processing import filter_list_of_strings
#
print(filter_list_of_strings(["aaaaaaaaabbbbb", "ccccccccbbbbb"], 4))

def test_filter_list_of_strings() -> None:
    substring: str = "cccccc"
    strings: list[str] = ["aaaaaaaa", "bbbbbbb"]
    test_input: list[str] = [string + substring for string in strings]
    assert filter_list_of_strings(test_input, 4) == strings