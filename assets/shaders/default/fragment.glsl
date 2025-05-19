#version 330 core
#define PI 3.1415926538
out vec4 FragColor;

in vec4 vCol;
in vec2 uv;

uniform float time;
uniform sampler2D ourTexture;

void main()
{
FragColor = texture(ourTexture, uv);
//FragColor = vec4(uv, 0.0, 1.0);
}
