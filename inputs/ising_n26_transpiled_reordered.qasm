// Generated from Cirq v1.1.0

OPENQASM 2.0;
include "qelib1.inc";


// Qubits: [q_0, q_1, q_2, q_3, q_4, q_5, q_6, q_7, q_8, q_9, q_10, q_11, q_12, q_13, q_14, q_15, q_16, q_17, q_18, q_19, q_20, q_21, q_22, q_23, q_24, q_25]
qreg q[26];


rz(pi*0.5) q[0];
rz(pi*0.5) q[1];
rz(pi*0.5) q[2];
rz(pi*0.5) q[3];
rz(pi*0.5) q[4];
rz(pi*0.5) q[5];
rz(pi*0.5) q[6];
rz(pi*0.5) q[7];
rz(pi*0.5) q[8];
rz(pi*0.5) q[9];
rz(pi*0.5) q[10];
rz(pi*0.5) q[11];
rz(pi*0.5) q[12];
rz(pi*0.5) q[13];
rz(pi*0.5) q[14];
rz(pi*0.5) q[15];
rz(pi*0.5) q[16];
rz(pi*0.5) q[17];
rz(pi*0.5) q[18];
rz(pi*0.5) q[19];
rz(pi*0.5) q[20];
rz(pi*0.5) q[21];
rz(pi*0.5) q[22];
rz(pi*0.5) q[23];
rz(pi*0.5) q[24];
rz(pi*0.5) q[25];
sx q[0];
sx q[1];
sx q[2];
sx q[3];
sx q[4];
sx q[5];
sx q[6];
sx q[7];
sx q[8];
sx q[9];
sx q[10];
sx q[11];
sx q[12];
sx q[13];
sx q[14];
sx q[15];
sx q[16];
sx q[17];
sx q[18];
sx q[19];
sx q[20];
sx q[21];
sx q[22];
sx q[23];
sx q[24];
sx q[25];
rz(pi*-0.0577241514) q[0];
rz(pi*-0.3845517014) q[1];
rz(pi*0.1482883943) q[2];
rz(pi*-0.7965767927) q[3];
rz(pi*0.2693250664) q[4];
rz(pi*0.9613498735) q[5];
rz(pi*0.0451788139) q[6];
rz(pi*-0.5903576321) q[7];
rz(pi*-0.9667949142) q[8];
rz(pi*-0.5664101926) q[9];
rz(pi*0.8301986564) q[10];
rz(pi*-0.1603973289) q[11];
rz(pi*0.9152275349) q[12];
rz(pi*-0.3304550954) q[13];
rz(pi*0.7359662614) q[14];
rz(pi*0.0280674603) q[15];
rz(pi*0.7747311216) q[16];
rz(pi*-0.0494622592) q[17];
rz(pi*0.1444728773) q[18];
rz(pi*-0.7889457588) q[19];
rz(pi*0.2099820991) q[20];
rz(pi*-0.9199642088) q[21];
rz(pi*0.1311669511) q[22];
rz(pi*-0.7623339064) q[23];
rz(pi*0.7302624029) q[24];
rz(pi*0.0394751655) q[25];
cx q[0],q[1];
cx q[2],q[3];
cx q[4],q[5];
cx q[6],q[7];
cx q[8],q[9];
cx q[10],q[11];
cx q[12],q[13];
cx q[14],q[15];
cx q[16],q[17];
cx q[18],q[19];
cx q[20],q[21];
cx q[22],q[23];
cx q[24],q[25];
rz(pi*-0.5577241524) q[1];
rz(pi*-0.3517116068) q[3];
rz(pi*-0.2306749346) q[5];
rz(pi*-0.4548211871) q[7];
rz(pi*0.533205092) q[9];
rz(pi*0.3301986649) q[11];
rz(pi*0.4152275434) q[13];
rz(pi*0.2359662699) q[15];
rz(pi*0.2747311301) q[17];
rz(pi*-0.3555271237) q[19];
rz(pi*-0.2900179019) q[21];
rz(pi*-0.3688330499) q[23];
rz(pi*0.2302624177) q[25];
cx q[0],q[1];
cx q[2],q[3];
cx q[4],q[5];
cx q[6],q[7];
cx q[8],q[9];
cx q[10],q[11];
cx q[12],q[13];
cx q[14],q[15];
cx q[16],q[17];
cx q[18],q[19];
cx q[20],q[21];
cx q[22],q[23];
cx q[24],q[25];
rz(pi*0.1949971106) q[1];
rz(pi*-0.3899942275) q[2];
rz(pi*-0.2699771656) q[3];
rz(pi*0.5399543439) q[4];
rz(pi*0.2857217434) q[5];
rz(pi*-0.5714434995) q[6];
rz(pi*0.2659680589) q[7];
rz(pi*-0.5319361178) q[8];
rz(pi*-0.2489895115) q[9];
rz(pi*0.4979790102) q[10];
rz(pi*-0.3201533779) q[11];
rz(pi*0.6403067558) q[12];
rz(pi*-0.3881761687) q[13];
rz(pi*0.7763523375) q[14];
rz(pi*0.4685326719) q[15];
rz(pi*-0.9370653438) q[16];
rz(pi*-0.2936297896) q[17];
rz(pi*0.5872595856) q[18];
rz(pi*-0.4908431073) q[19];
rz(pi*0.9816862146) q[20];
rz(pi*0.0406487359) q[21];
rz(pi*-0.0812974717) q[22];
rz(pi*0.2145129953) q[23];
rz(pi*-0.4290259905) q[24];
cx q[1],q[2];
cx q[3],q[4];
cx q[5],q[6];
cx q[7],q[8];
cx q[9],q[10];
cx q[11],q[12];
cx q[13],q[14];
cx q[15],q[16];
cx q[17],q[18];
cx q[19],q[20];
cx q[21],q[22];
cx q[23],q[24];
rz(pi*0.1949971106) q[2];
rz(pi*-0.2699771656) q[4];
rz(pi*0.2857217434) q[6];
rz(pi*0.2659680589) q[8];
rz(pi*-0.2489895115) q[10];
rz(pi*-0.3201533779) q[12];
rz(pi*-0.3881761687) q[14];
rz(pi*0.4685326719) q[16];
rz(pi*-0.2936297896) q[18];
rz(pi*-0.4908431073) q[20];
rz(pi*0.0406487359) q[22];
rz(pi*0.2145129953) q[24];
cx q[1],q[2];
cx q[3],q[4];
cx q[5],q[6];
cx q[7],q[8];
cx q[9],q[10];
cx q[11],q[12];
cx q[13],q[14];
cx q[15],q[16];
cx q[17],q[18];
cx q[19],q[20];
cx q[21],q[22];
cx q[23],q[24];