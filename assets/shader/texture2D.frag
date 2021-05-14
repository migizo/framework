#version 330 core



out vec4 fragColor;

uniform vec2 iResolution;
uniform sampler2D uScreenTexture;

void main()
{
    vec2 uv = gl_FragCoord.xy /(iResolution.xy);
    fragColor = texture(uScreenTexture, uv.xy);
}