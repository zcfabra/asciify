#!/usr/bin/python3

import cv2
import sys
import numpy as np
from tqdm import tqdm
def main():
    args = sys.argv
    vals = ".*%#."
    num_intervals = len(vals)
    interval = 255 /num_intervals 
    scale_down = 2


    cap = cv2.VideoCapture(args[1])
    while (pbar:= tqdm(cap.isOpened())):
        ret, frame = cap.read()
        if ret == True:
            
            bw_frame = np.mean(frame, axis=2)
            print(bw_frame.shape[0], bw_frame.shape[1])
            # bw_frame_resized = cv2.resize(bw_frame, (bw_frame.shape[0] // scale_down, bw_frame.shape[1] // scale_down))
            bw_frame_resized = bw_frame
            bw_frame_resized[bw_frame_resized == 255] = 0
            out = ""
            for i in range(bw_frame_resized.shape[0]):
                for j in range(bw_frame_resized.shape[1]):
                    left_over = bw_frame_resized[i][j]
                    which_char = 0
                    while left_over - interval > 0:
                        left_over -= interval
                        which_char +=1
                    out+=vals[which_char]
                out+='\n'

            font = cv2.FONT_HERSHEY_SIMPLEX

            image = np.zeros((bw_frame_resized.shape[1]*10,bw_frame_resized.shape[0]*10))
            # org
            
            # fontScale
            fontScale = 0.5
            
            # Blue color in BGR
            color = (255, 255, 255)
            
            # Line thickness of 2 px
            thickness = 0
            
            # Using cv2.putText() method
            for ix, line in enumerate(lines:= out.split("\n")):
                x = 10
                y= (ix+1) * 10
                image = cv2.putText(image, line, (x,y), font, 
                    fontScale, color, thickness, cv2.LINE_AA)
            img = cv2.imwrite("out.png", image)
            break
            


        else:
            break
    cap.release()


if __name__ == "__main__":
    main()