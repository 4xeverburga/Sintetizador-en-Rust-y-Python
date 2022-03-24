import numpy as np
import scipy
from scipy.io.wavfile import write
# Generate a wav file from the .syv table
samplerate = 44100
file = open("playback_table.syv", "r")
lines = file.readlines()
file.close()
data = np.array([float(line) for line in lines])
write("example.wav", samplerate, data.astype(np.float32))