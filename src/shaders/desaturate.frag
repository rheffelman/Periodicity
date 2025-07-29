uniform sampler2D texture;
uniform float desaturation;

void main() {
    vec4 color = texture2D(texture, gl_TexCoord[0].xy);
    float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    color.rgb = mix(color.rgb, vec3(gray), desaturation);
    gl_FragColor = color;
}
