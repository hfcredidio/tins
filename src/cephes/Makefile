CC = gcc
CFLAGS = -O3 -fno-builtin -fPIC
AR = ar
RANLIB = ranlib
INCS = mconf.h
AS = as

OBJS = acosh.o airy.o asin.o asinh.o atan.o atanh.o bdtr.o beta.o \
btdtr.o cbrt.o chbevl.o chdtr.o clog.o cmplx.o const.o \
cosh.o dawsn.o drand.o ei.o ellie.o ellik.o ellpe.o ellpj.o ellpk.o \
exp.o exp10.o exp2.o expn.o expx2.o fabs.o fac.o fdtr.o \
fresnl.o gamma.o gdtr.o hyp2f1.o hyperg.o i0.o i1.o igami.o incbet.o \
incbi.o igam.o isnan.o iv.o j0.o j1.o jn.o jv.o k0.o k1.o kn.o kolmogorov.o \
log.o log2.o log10.o lrand.o nbdtr.o ndtr.o ndtri.o pdtr.o planck.o \
polevl.o polmisc.o polylog.o polyn.o pow.o powi.o psi.o rgamma.o round.o \
shichi.o sici.o sin.o sindg.o sinh.o spence.o stdtr.o struve.o \
tan.o tandg.o tanh.o unity.o yn.o zeta.o zetac.o \
sqrt.o floor.o setprec.o mtherr.o

OBJ := $(addprefix $(OUT_DIR)/, $(OBJS))

default: $(OUT_DIR)/libmd.a

$(OUT_DIR)/%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(OUT_DIR)/beta.o: beta.c
	$(CC) $(CFLAGS) -c $< -o $@_
	$(OUT_DIR)/objconv -nr:beta:_ceph_beta $@_ $@

$(OUT_DIR)/psi.o: psi.c
	$(CC) $(CFLAGS) -c $< -o $@_
	$(OUT_DIR)/objconv -nr:psi:_ceph_psi $@_ $@

$(OUT_DIR)/hyp2f1.o: hyp2f1.c
	$(CC) $(CFLAGS) -c $< -o $@_
	$(OUT_DIR)/objconv -nr:psi:_ceph_psi $@_ $@

$(OUT_DIR)/libmd.a: $(OBJ) $(INCS)
	$(AR) rv $(OUT_DIR)/libmd.a $(OBJ)
	$(RANLIB) $(OUT_DIR)/libmd.a

clean:
	rm -f $(OUT_DIR)/*.o
	rm -f $(OUT_DIR)/libmd.a

