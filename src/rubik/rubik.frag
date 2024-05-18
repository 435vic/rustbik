in vec4 col;
in vec3 nor;
in vec2 uvs;
in vec3 pos;

uniform vec2 stickerData;
uniform vec3 cameraPosition;

layout (location = 0) out vec4 outColor;

vec4 sticker(vec4 colors, vec2 uv) {
    vec2 shifted = uv*2.0 - vec2(1, 1);
    if ((abs(shifted.x) < stickerData.x && abs(shifted.y) < stickerData.x + stickerData.y) ||
        (abs(shifted.x) < stickerData.x + stickerData.y && abs(shifted.y) < stickerData.x) ||
        dot(abs(shifted) - vec2(stickerData.x, stickerData.x), abs(shifted) - vec2(stickerData.x, stickerData.x)) < stickerData.y * stickerData.y) {
        return colors;
    }
    return vec4(0.0, 0.0, 0.0, 1.0);
}

void main() {
    vec4 surface_color = sticker(col, uvs);
    //vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    // outColor.rgb = calculate_lighting(cameraPosition, surface_color.rgb, pos, normal, 0.0, 1.0, 1.0);
    outColor.rgb = surface_color.rgb;
    outColor.a = 1.0;
    // outColor.rgb = outColor.rgb * cel_shading(vec3(1.0, 1.0, 1.0), normalize(vec3(1.0, 1.0, 1.0)), normal, vec3(0.0, 1.0, 0.0));
    // outColor.rgb = tone_mapping(outColor.rgb);
    // outColor.rgb = color_mapping(outColor.rgb);
}