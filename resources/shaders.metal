#include <metal_stdlib>

using namespace metal;

struct UiVertexStruct {
	float4 position;
};

struct UiFragmentStruct {
    float4 position [[position]];
};

// vertex shader function
vertex UiFragmentStruct vertex_ui(device UiVertexStruct* vertexArray [[ buffer(0) ]],
                                  device float4 colour [[ buffer(1) ]]
                                  unsigned int vid [[ vertex_id ]])
{
    UiFragmentStruct out;
    out.position = vertexArray[vid].position;
    return out;
}

// fragment shader function
fragment float4 fragment_ui(UiFragmentStruct in [[stage_in]])
{
    return float4(1.0, 1.0, 1.0, 1.0);
}
