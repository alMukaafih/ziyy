import re

# The input string
input_string = "rgb(255, 128, 0)"

# Define a regular expression pattern to match "rgb(*, *, *)"
pattern = r'rgb\(\d+, \d+, \d+\)'

# Use re.search to find a match in the input string
match = re.search(pattern, input_string)

# Check if a match was found
if match:
    print("Match found:", match.group())
else:
    print("No match found")
