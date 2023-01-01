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
   print(img_data)
   print(img_data.shape)
   num_ranges = 5
   interval = img_data.max() - img_data.min() / num_ranges
   bw = img_data.mean(axis=2)
   print(bw.shape)
   bw[bw==255] = 0
   bw = Image.fromarray(bw.astype(np.uint8))

   bw.save("hi.png")
if __name__ == "__main__":
    main()