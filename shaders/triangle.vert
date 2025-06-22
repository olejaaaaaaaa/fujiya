#version 450

// Квадрат из двух треугольников (6 вершин)
vec2 positions[6] = vec2[](
    vec2(-1.0, -1.0),  // 1-й треугольник
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),
    
    vec2( 1.0, -1.0),  // 2-й треугольник
    vec2( 1.0,  1.0),
    vec2(-1.0,  1.0)
);

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}