#include <metal_stdlib>

using namespace metal;

struct VertexStruct {
	float4 position;
	float2 textureCoords;
};

struct FragmentStruct {
    float4 position [[position]];
    float2 textureCoords;
};

// vertex shader function
vertex FragmentStruct vertex_static(device VertexStruct* vertexArray [[ buffer(0) ]],
                                    unsigned int vid [[ vertex_id ]])
{
    FragmentStruct out;
    out.position = vertexArray[vid].position;
    out.textureCoords = vertexArray[vid].textureCoords;
    return out;
}

// fragment shader function
fragment float4 fragment_static(FragmentStruct in [[stage_in]],
                                texture2d<float> texture [[texture(0)]],
                                sampler sam [[sampler(0)]])
{
    return texture.sample(sam, in.textureCoords);
}
