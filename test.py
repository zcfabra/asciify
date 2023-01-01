from PIL import Image
import cv2
import sys
import matplotlib.pyplot as plt
import numpy as np
# np.set_printoptions(threshold=sys.maxsize)

def main():

   args = sys.argv

   file_name = args[1]

   img_data = cv2.imread(filename=file_name)
   num_ranges = 5
   bw = img_data.mean(axis=2)
   bw[bw==255] = 0
   bw =  bw.astype(np.uint8)
   resized = cv2.resize(bw, (200,200))
   interval =( resized.max() - resized.min()) / num_ranges
   unique, counts = np.unique(resized, return_counts=True)
#    for each, count in zip(unique, counts):
#     print(each, count)


   scales = " .*%#"
   out = ""

   for i in range(200):
    for j in range(200):
        left_over = resized[i][j]
        which_interval = 0
        while left_over - interval >0:
            left_over -=interval
            which_interval+=1
        # print(which_interval)
        out+=scales[which_interval]
    out+='\n'

    # print(out)
    with open("samps/out.txt", "w") as f:
        f.write(out)


        




#    bw.save("samps/hi.png")

if __name__ == "__main__":
    main()