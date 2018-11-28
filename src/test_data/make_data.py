import os
import scipy.special as sp
import numpy as np
import array


def f64_airy_a():
    _x = 10 ** np.linspace(-3, 3, 100)
    x = np.concatenate(([-np.inf], _x, [0.0], _x, [np.inf, np.nan]))
    y = sp.airy(x)[0]
    data = np.array(zip(x, y)).ravel();
    here = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(here, 'f64_airy_a')
    np.save(file_path, data)


def c64_airy_a():
    _x = 10 ** np.linspace(-3, 3, 100)
    x = np.concatenate(([-np.inf], _x, [0.0], _x, [np.inf, np.nan]))
    xx, yy = np.meshgrid(x, x)
    z = xx.ravel() + yy.ravel() * 1j
    y = sp.airy(z)[0]
    data = np.concatenate((z.real, z.imag, y.real, y.imag))
    here = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(here, 'c64_airy_a')
    np.save(file_path, data)


if __name__ == '__main__':
    f64_airy_a()
    c64_airy_a()
