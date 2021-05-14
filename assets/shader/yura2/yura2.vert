#version 330 core

layout (location = 0) in vec3 Position;

uniform mat4 uModelMat;
uniform mat4 uViewMat;
uniform mat4 uProjectionMat;

void main()
{
    vec3 newPos = Position;
    gl_Position = vec4(newPos, 1.0);
}