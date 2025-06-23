from string_processing import filter_list_of_strings


def test_filter_list_of_strings_removes_substring() -> None:
    substring: str = "cccccc"
    strings: list[str] = ["aaaaaaaa", "bbbbbbb"]
    test_input: list[str] = [string + substring for string in strings]
    assert filter_list_of_strings(test_input, 4) == strings
    assert strings != test_input


def test_filter_list_of_strings_keeps_substring() -> None:
    substring: str = "cccccc"
    strings: list[str] = ["aaaaaaaa", "bbbbbbb"]
    test_input: list[str] = [string + substring for string in strings]
    assert filter_list_of_strings(test_input, 10) == test_input
    assert strings != test_input
