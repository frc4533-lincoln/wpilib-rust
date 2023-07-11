# get cwd
import os

cwd = os.getcwd()

# write to file
with open("scripts\\TOP_DIR_PATH", "w") as f:
    f.write(cwd)
