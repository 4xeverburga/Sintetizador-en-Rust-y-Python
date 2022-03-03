import numpy as np
import scipy
from scipy.io.wavfile import write
samplerate = 44100
file = open("playback_table.syv", "r")
lines = file.readlines()

data = np.array([float(line) for line in lines])
write("example.wav", samplerate, data.astype(np.float32))