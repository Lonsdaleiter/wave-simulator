#include <metal_stdlib>

using namespace metal;

struct TextVertexStruct {
	float2 position;
	float2 textureCoords;
};

struct TextInstanceStruct {
    float2 translation;
};

struct TextFragmentStruct {
    float4 position [[position]];
    float2 textureCoords;
};

vertex TextFragmentStruct text_vert(device TextVertexStruct *vertexArray [[ buffer(0) ]],
                                    constant TextInstanceStruct &perInstance [[ buffer(1) ]],
                                    unsigned int vid [[ vertex_id ]])
{
    TextFragmentStruct out;
    out.position = float4(vertexArray[vid].position, 0.0, 1.0);
    out.textureCoords = vertexArray[vid].textureCoords;
    return out;
}

fragment float4 text_frag(TextFragmentStruct in [[stage_in]],
                          constant float3 &color [[ buffer(0) ]],
                          texture2d<float> texture [[texture(0)]],
                          sampler sam [[sampler(0)]])
{
    return float4(color, texture.sample(sam, in.textureCoords).a);
    // return float4(color, 1.0);
    // return float4(in.textureCoords, 0.0, 1.0);
    // return texture.sample(sam, (in.position.x + 1.0) / 2.0, (-in.position.y + 1.0) / 2.0);
}
