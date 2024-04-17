#all files data in one file in outputs/
#print the data to one file. and name lable the data with the file name

import os
import sys

folder_path = 'outputs/'

for filename in os.listdir(folder_path):
    file_path = os.path.join(folder_path, filename)
    if os.path.isfile(file_path):
        with open(file_path, 'r') as file:
            with open('outputs/final_output.txt', 'a') as f:
                f.write(f"{filename}\n")
                for l in file:
                    f.write(f"{l}")
                f.write("\n---------------------------------\n")