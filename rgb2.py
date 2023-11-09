import re

# The input string
input_string = "rgb(255,128, 0)"

# Define a regular expression pattern to match "rgb(*, *, *)"
#pattern = r'rgb\((\d+),(\d+),(\d+)\)'
pattern = r'rgb\((.+)\)'

# Use re.findall to extract the values as a list of tuples
matches = re.findall(pattern, input_string)
matches = re.sub(r'\,', ';', matches[0])
matches = re.sub(r' ', '', matches)
print(matches)

# Check if any matches were found
#if matches:
    # Extract the values from the first match
    #values = matches[0]
    #r, g, b = map(int, values)
    #print(f"Red: {r}, Green: {g}, Blue: {b}")
#else:
#    print("No match found")
