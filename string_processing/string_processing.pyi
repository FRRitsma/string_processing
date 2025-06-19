def filter_list_of_strings(strings: list[str], min_size: int) -> list[str]:
    """
    Removes shared substrings that are longer than length min_size.

    Args:
        strings: Input list of strings
        min_size: Remove substrings larger than min_size

    Example:
        >>> filter_list_of_strings(["aaaabbb", "aaaaccc"], 3)
        ['bbb', 'ccc']
    """
    ...
