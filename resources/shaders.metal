#include <metal_stdlib>

using namespace metal;

struct TextVertexStruct {
	float4 position;
};

struct TextInstanceStruct {
    float4x4 transformation;
};

struct TextFragmentStruct {
    float4 position [[position]];
};

vertex TextFragmentStruct text_vert(device TextVertexStruct *vertexArray [[ buffer(0) ]],
                                    constant TextInstanceStruct &perInstance [[ buffer(1) ]],
                                    unsigned int vid [[ vertex_id ]])
{
    float4 newPosition = perInstance.transformation * vertexArray[vid].position;

    TextFragmentStruct out;
    out.position = newPosition;
    return out;
}

fragment float4 text_frag(TextFragmentStruct in [[stage_in]],
                          constant float4 &color [[ buffer(0) ]])
{
    return color;
}
