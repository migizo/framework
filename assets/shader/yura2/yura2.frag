#version 330 core
#define TWO_PI 6.2824
vec3 mod289(vec3 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 mod289(vec4 x) {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 permute(vec4 x) {
     return mod289(((x*34.0)+1.0)*x);
}

vec4 taylorInvSqrt(vec4 r)
{
  return 1.79284291400159 - 0.85373472095314 * r;
}

float snoise(vec3 v)
  { 
  const vec2  C = vec2(1.0/6.0, 1.0/3.0) ;
  const vec4  D = vec4(0.0, 0.5, 1.0, 2.0);

// First corner
  vec3 i  = floor(v + dot(v, C.yyy) );
  vec3 x0 =   v - i + dot(i, C.xxx) ;

// Other corners
  vec3 g = step(x0.yzx, x0.xyz);
  vec3 l = 1.0 - g;
  vec3 i1 = min( g.xyz, l.zxy );
  vec3 i2 = max( g.xyz, l.zxy );

  //   x0 = x0 - 0.0 + 0.0 * C.xxx;
  //   x1 = x0 - i1  + 1.0 * C.xxx;
  //   x2 = x0 - i2  + 2.0 * C.xxx;
  //   x3 = x0 - 1.0 + 3.0 * C.xxx;
  vec3 x1 = x0 - i1 + C.xxx;
  vec3 x2 = x0 - i2 + C.yyy; // 2.0*C.x = 1/3 = C.y
  vec3 x3 = x0 - D.yyy;      // -1.0+3.0*C.x = -0.5 = -D.y

// Permutations
  i = mod289(i); 
  vec4 p = permute( permute( permute( 
             i.z + vec4(0.0, i1.z, i2.z, 1.0 ))
           + i.y + vec4(0.0, i1.y, i2.y, 1.0 )) 
           + i.x + vec4(0.0, i1.x, i2.x, 1.0 ));

// Gradients: 7x7 points over a square, mapped onto an octahedron.
// The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
  float n_ = 0.142857142857; // 1.0/7.0
  vec3  ns = n_ * D.wyz - D.xzx;

  vec4 j = p - 49.0 * floor(p * ns.z * ns.z);  //  mod(p,7*7)

  vec4 x_ = floor(j * ns.z);
  vec4 y_ = floor(j - 7.0 * x_ );    // mod(j,N)

  vec4 x = x_ *ns.x + ns.yyyy;
  vec4 y = y_ *ns.x + ns.yyyy;
  vec4 h = 1.0 - abs(x) - abs(y);

  vec4 b0 = vec4( x.xy, y.xy );
  vec4 b1 = vec4( x.zw, y.zw );

  //vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
  //vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
  vec4 s0 = floor(b0)*2.0 + 1.0;
  vec4 s1 = floor(b1)*2.0 + 1.0;
  vec4 sh = -step(h, vec4(0.0));

  vec4 a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
  vec4 a1 = b1.xzyw + s1.xzyw*sh.zzww ;

  vec3 p0 = vec3(a0.xy,h.x);
  vec3 p1 = vec3(a0.zw,h.y);
  vec3 p2 = vec3(a1.xy,h.z);
  vec3 p3 = vec3(a1.zw,h.w);

//Normalise gradients
  vec4 norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2, p2), dot(p3,p3)));
  p0 *= norm.x;
  p1 *= norm.y;
  p2 *= norm.z;
  p3 *= norm.w;

// Mix final noise value
  vec4 m = max(0.6 - vec4(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), 0.0);
  m = m * m;
  return 42.0 * dot( m*m, vec4( dot(p0,x0), dot(p1,x1), 
                                dot(p2,x2), dot(p3,x3) ) );
  }


out vec4 fragColor;

uniform float iTime;
uniform vec2 iResolution;

const float delta = 0.1f;
const vec3 deltaX = vec3(delta, 0, 0);
const vec3 deltaY = vec3(0, delta, 0);
const vec3 deltaZ = vec3(0, 0, delta);

float map(float vin, float vinMin, float vinMax, float voutMin, float voutMax) {
  return (vin - vinMin) / (vinMax - vinMin) * (voutMax - voutMin) + voutMin;
}

vec3 trefoilKnot(float t, float radius) {
  return vec3(sin(t) + 2 * sin(2*t), cos(t) - 2 * cos(2*t), -sin(3*t)) * radius;
}

mat4 rotateX(float theta) {
  float c = cos(theta);
  float s = sin(theta);
  return mat4(
    1, 0, 0, 0,
    0, c, -s, 0,
    0, s, c, 0,
    0, 0, 0, 1
  );
}

mat4 rotateY(float theta) {
  float c = cos(theta);
  float s = sin(theta);
  return mat4(
    c, 0, s, 0,
    0, 1, 0, 0,
    -s, 0, c, 0,
    0, 0, 0, 1
  );
}

mat4 rotateZ(float theta) {
  float c = cos(theta);
  float s = sin(theta);
  return mat4(
    c, -s, 0, 0,
    s, c, 0, 0,
    0, 0, 1, 0,
    0, 0, 0, 1
  );
}

float sdSphere(vec3 p, float radius) {
	return length(p) - radius;   
}

float sdOctahedron( vec3 p, float s)
{
  p = abs(p);
  return (p.x+p.y+p.z-s)*0.57735027;
}

float opSmoothUnion( float d1, float d2, float k ) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h); }

float sd(vec3 p) {
    float nearestDist = 9999.0;

    float minRadius = 0.3;
    float maxRadius = 0.5;

    float n = sin(iTime) * 0.5 + 0.5;
    float radius = map(n, 1, 0, minRadius, maxRadius);

    p = (rotateY(0.5) * rotateX(0.5) * vec4(p, 1)).xyz;
    for (int i = 0; i < 3; i++) {
        vec3 objPos;
        objPos = trefoilKnot(float(i) / 3 * TWO_PI + iTime * 0.4, 0.4);
        objPos.z = 0;
        nearestDist = opSmoothUnion(nearestDist, sdSphere(p + objPos, radius), 1);
    }
    return nearestDist;
}

vec3 sdNormal(vec3 p) {
  vec3 normal;
  normal.x = sd(p - deltaX) - sd(p);
  normal.y = sd(p - deltaY) - sd(p);
  normal.z = sd(p - deltaZ) - sd(p);
	return normalize(normal); 
}

vec4 rayMarching(vec4 bgColor) {
  vec2 uv = (gl_FragCoord.xy * 2.0f - iResolution.xy) /max(iResolution.x, iResolution.y);
  uv.x += sin(iTime*10 + uv.y * 30) * (sin(iTime)*0.5+0.5) * 0.001;
  vec3 camera_pos = vec3(0, 0, -5);
  vec3 camera_dir = vec3(0, 0, 1);
  vec3 camera_up = vec3(0, 1, 0);
  vec3 camera_right = normalize(cross(camera_up, camera_dir));
  
  vec3 ray_pos = camera_pos;
  vec3 ray_dir = normalize(camera_dir + uv.x * camera_right + uv.y * camera_up);

  float EPS = 0.01f;
  int countMax = 30;

  float depth = 0;
  vec4 col = bgColor;
  for (int i = 0; i < countMax; i++) {
    float dist = sd(ray_pos);
        //  dist += 0.1 * snoise(sdNormal(ray_pos) + vec3(iTime));

    if (dist < EPS) {
        vec3 normal = sdNormal(ray_pos);

      col.xyz = mix(normal, vec3(1), 0.8);
      //  float fog = 1.0 - exp(-depth * 0.05);
      // col.xyz = mix(col.xyz, vec3(1), fog);
      break;   
    }
    depth += dist;
    ray_pos += ray_dir * dist;
  }
  return col;
}

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main()
{
    vec2 uv = gl_FragCoord.xy /min(iResolution.x, iResolution.y);
    float t = iTime * 0.2;
    vec3 sx = vec3(uv.xy, t);
    vec3 sy = vec3(uv.yx, t + 9898);
    vec4 noiseColor = vec4(snoise(sx), snoise(sy), 1.0, 1.0);
    noiseColor.xyz = mix(noiseColor.xyz, vec3(1), 0.8);
    fragColor = rayMarching(noiseColor);
}