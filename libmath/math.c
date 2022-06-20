// Math Functions Located In ASM.S
extern float asm_sqrt(float x);
extern float asm_atan2(float x, float y);
extern float asm_pow(float x, float y);
extern float asm_sin(float x);
extern float asm_cos(float x);
extern float asm_remap(float x, float y, float z, float i, float j);

// Wrapper Functions For ASM.S
float c_sqrt(float x) { return asm_sqrt(x); }
float c_atan2(float x, float y) { return asm_atan2(x, y); }
float c_pow(float x, float y) { return asm_pow(x, y); }
float c_sin(float x) { return asm_sin(x); }
float c_cos(float x) { return asm_cos(x); }
float c_remap(float x, float y, float z, float i, float j) { return asm_remap(x, y, z, i, j); }