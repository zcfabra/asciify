#!/usr/bin/python3

import cv2
import sys
import numpy as np
from tqdm import tqdm
import imgkit
def main():
    args = sys.argv
    vals = " .*%# "
    num_intervals = len(vals)
    interval = 255 /num_intervals 
    scale_down = 2

    fourcc = cv2.VideoWriter_fourcc(*'mp4v')
    cap = cv2.VideoCapture(args[1])
    fps = cap.get(cv2.CAP_PROP_FPS)
    print(f"FPS: {fps}")
    frames = []
    while (cap.isOpened()):
        ret, frame = cap.read()
        if ret == True:
            
            bw_frame = np.mean(frame, axis=2)
            # print("Shape", bw_frame.shape[0], bw_frame.shape[1])
            bw_frame_resized = cv2.resize(bw_frame, (400,200))
            # print("Resized", bw_frame_resized.shape)
            # bw_frame_resized = bw_frame
            bw_frame_resized[bw_frame_resized == 255] = 0
            out = ""
            for i in range(bw_frame_resized.shape[0]):
                for j in range(bw_frame_resized.shape[1]):
                    left_over = bw_frame_resized[i][j]
                    which_char = 0
                    while left_over - interval > 0:
                        left_over -= interval
                        which_char +=1
                    out+=f"<span >{vals[which_char]}</span>"
                out+='\n'

            out_html = f"<html><body style=\"background-color: #000000\"><pre style=\"display: inline-block; border-width: 4px 6px; border-color: black; color: #00ff00; font-size: 10px; line-height:10px\">{out}</pre></body></html>"
            imgkit.from_string(out_html,"out.jpg" )

            frames.append(cv2.imread("out.jpg"))





            


        else:
            break
    vid = cv2.VideoWriter("output.mp4", fourcc, fps, (frames[0].shape[0], frames[0].shape[1]), True)
    for frame in frames:
        vid.write(frame)

    cap.release()
    vid.release()



if __name__ == "__main__":
    main()