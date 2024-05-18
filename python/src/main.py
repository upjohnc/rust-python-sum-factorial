from pathlib import Path

from rust_factorial_sum import file_sum_factorial


def main():
    """
    Calculate the sum of factorials for each file in the input_files dir.
    If the first line in the file is not an integer than save `None`
    """
    list_files = [i for i in Path("./input_files").glob("*") if i.is_file()]
    result = {}
    for i in list_files:
        try:
            value = file_sum_factorial(str(i.resolve()))
        except:
            # except Exception as  e:
            # print(e)
            value = None
        result[str(i)] = value
    print(result)


if __name__ == "__main__":
    main()
