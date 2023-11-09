my_list = [10, 20, 30, 40, 50]

# Check if the list has an index
index_to_check = 5  # Change this to the index you want to check

if 0 <= index_to_check < len(my_list):
    print(f"The list has an element at index {index_to_check}: {my_list[index_to_check]}")
else:
    print(f"The list does not have an element at index {index_to_check}")

