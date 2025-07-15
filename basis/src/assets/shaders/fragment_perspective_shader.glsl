#version 330 core
out vec4 FragColor;

in vec3 Color;
in vec2 TexCoord;
in float FaceId;
in float MaxFaceId;

uniform sampler2D object_texture;

void main()
{
    vec4 texture_color = texture(object_texture, TexCoord);

    float color_percentage = 1.0;
    float r = FaceId / MaxFaceId;
    float g = FaceId / MaxFaceId;
    float b = FaceId / MaxFaceId;
    vec4 color = vec4(r, g, b, 1.0);

    FragColor = texture_color * color * color_percentage;
}
