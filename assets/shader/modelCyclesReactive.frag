#version 330 core

out vec4 fragColor;

uniform vec2 iResolution;
uniform float iTime;

uniform float iFreq;
uniform float T[6];

float map(float _in, float inMin, float inMax, float outMin, float outMax) {
  return ((_in - inMin) / (inMax - inMin)) * (outMax - outMin) + outMin;
}
void main()
{
    vec2 uv = gl_FragCoord.xy /min(iResolution.x, iResolution.y); // 0 to 1
    uv.y = 1.0 - uv.y;
    
    vec3 bgColor = vec3(1, 0.9, 0.8);
    vec3 primaryColor = vec3(0.8, 0.36, 0.36);
    vec3 subColor = vec3(0.37, 0.61, 0.61);
    vec4 newColor = vec4(bgColor, 1);
    float minRadius = 0.1;
    float maxRadius = 0.4;

    vec2 p = uv - vec2(0.5); // -0.5 ~ 0.5

    // circle
    for (int i = 0; i < 6; i++) {
      float radius = map(float(i), 0.0, 5.0, minRadius, maxRadius);

      int thetaNum = 20;
      for (int j = 0; j < thetaNum; j++) {
        float theta = iTime * 0.4 + i * 0.2 + float(j) / float(thetaNum) * 6.2824 ;
        vec2 c;
        c.x = cos(theta) * radius;
        c.y = sin(theta) * radius;

        float circleRadius = 0.02;
        float fadeRange = 0.01;
        float dist = distance(c, p);
        float circle = smoothstep(circleRadius + fadeRange, circleRadius, dist);
        newColor *= vec4(vec3(1.0) - primaryColor, 0);
      }
    }

    // line
    // float lineWidth = 0.01;
    // float hLineWidth = lineWidth / 2;
    // for (int i = 0; i < 6; i++) {
    //   float radius = map(float(i), 0.0, 5.0, minRadius, maxRadius);
    //   float dist = distance(vec2(0), p);
    //   float line = smoothstep(radius - hLineWidth, radius, dist);
    //   line *= smoothstep(radius + hLineWidth, radius, dist);

    //   newColor += vec4(line);
    // }
    
    fragColor = newColor;
}