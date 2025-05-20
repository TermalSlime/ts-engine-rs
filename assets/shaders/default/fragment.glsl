#version 330 core
#define PI 3.1415926538
out vec4 FragColor;

in vec4 vCol;
in vec2 uv;

uniform float time;
uniform sampler2D ourTexture;

void main()
{
    float red = (sin(time) / 2.0f) + 0.5f;
    float gre = (sin(time + PI * 2 * 0.33) / 2.0f) + 0.5f;
    float blu = (sin(time + PI * 2 * 0.66) / 2.0f) + 0.5f;
    FragColor = texture(ourTexture, uv) * vec4(red, gre, blu, vCol.a);
}
