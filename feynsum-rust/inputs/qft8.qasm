OPENQASM 2.0;
qreg q[8];
h q[7];
cphase(pi / 2) q[6], q[7];
cphase(pi / 4) q[5], q[7];
cphase(pi / 8) q[4], q[7];
cphase(pi / 16) q[3], q[7];
cphase(pi / 32) q[2], q[7];
cphase(pi / 64) q[1], q[7];
cphase(pi / 128) q[0], q[7];
h q[6];
cphase(pi / 2) q[5], q[6];
cphase(pi / 4) q[4], q[6];
cphase(pi / 8) q[3], q[6];
cphase(pi / 16) q[2], q[6];
cphase(pi / 32) q[1], q[6];
cphase(pi / 64) q[0], q[6];
h q[5];
cphase(pi / 2) q[4], q[5];
cphase(pi / 4) q[3], q[5];
cphase(pi / 8) q[2], q[5];
cphase(pi / 16) q[1], q[5];
cphase(pi / 32) q[0], q[5];
h q[4];
cphase(pi / 2) q[3], q[4];
cphase(pi / 4) q[2], q[4];
cphase(pi / 8) q[1], q[4];
cphase(pi / 16) q[0], q[4];
h q[3];
cphase(pi / 2) q[2], q[3];
cphase(pi / 4) q[1], q[3];
cphase(pi / 8) q[0], q[3];
h q[2];
cphase(pi / 2) q[1], q[2];
cphase(pi / 4) q[0], q[2];
h q[1];
cphase(pi / 2) q[0], q[1];
h q[0];
