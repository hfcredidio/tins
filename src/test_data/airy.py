from array import array

import numpy as np
import scipy.special as sp



def airy_a():
    x = 10 ** np.linspace(-5, 3, 100)
    x = np.concatenate(([-np.inf], -x[::-1], [0.0], x, [np.inf, np.nan]))
    y = sp.airy(x)[0]
    data = np.array(zip(x, y)).ravel()
    with open('./airy_a_f64.dat', 'wb') as file:
        array('d', data).tofile(file)

    x, y = np.meshgrid(x, x)
    x =  x + 1j * y
    y = sp.airy(x)[0]
    data = np.vstack((x.real, x.imag, y.real, y.imag)).T.ravel()
    with open('./airy_a_c64.dat', 'wb') as file:
        array('d', data).tofile(file)


if __name__ == "__main__":
    airy_a()
