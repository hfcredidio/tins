import os
import scipy.special as sp
import numpy as np
import array

x = np.linspace(-10, 10, 100)
y = sp.airy(x)[0]
data = np.concatenate((x, y))
here = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(here, 'airy_a.dat'), 'wb') as fout:
    array.array('d', data).tofile(fout)
