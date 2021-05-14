#version 330 core

out vec4 fragColor;

uniform vec2 iResolution;
uniform float iTime;
uniform vec2 iMouse;

uniform float iFreq;

void main()
{
    vec2 uv = gl_FragCoord.xy /(iResolution.xy); // 0 to 1
    uv.y = 1.0 - uv.y;
	  vec2 mouse = iMouse.xy/iResolution.xy;
    
    float dist = distance(mouse, uv);
    vec2 cuv = uv - mouse;
    float radial = sin((dist * iFreq * 0.02 - iTime * 2.0) * 6.2824);
    vec3 col = vec3(mix(1.0, radial, smoothstep(0.1, 0.4, dist)));
    col.r = (mix(1.0, sin((dist * iFreq * 0.02 - iTime * 2.0) * 6.2824 + 0.2), smoothstep(0.1, 0.4, dist)));
    fragColor = vec4(col, 1.0);
}