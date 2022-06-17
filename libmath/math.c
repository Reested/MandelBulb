#include <math.h>

float c_sqrt(float x) { return sqrtf(x); }
float c_atan2(float x, float y) { return atan2f(x, y); }
float c_pow(float x, float y) { return powf(x, y); }
float c_sin(float x) { return sin(x); }
float c_cos(float x) { return cos(x); }

float c_remap(float x, float y, float z, float i, float j) { return (i + (x - y) * (j - i) / (z - y)); }