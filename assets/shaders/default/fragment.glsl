#version 330 core
#define PI 3.1415926538
out vec4 FragColor;

in vec4 vCol;

uniform float time;

void main()
{
float red = (sin(time) / 2.0f) + 0.5f;
float gre = (sin(time + PI * 2 * 0.33) / 2.0f) + 0.5f;
float blu = (sin(time + PI * 2 * 0.66) / 2.0f) + 0.5f;
FragColor = vec4(red, gre, blu, vCol.a);
}
