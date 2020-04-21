#include <metal_stdlib>

using namespace metal;

struct UiVertexStruct {
	float4 position;
};

struct UiInstanceStruct {
    float4x4 transformation;
};

struct UiFragmentInStruct {
    float4 position [[position]];
};

vertex UiFragmentInStruct vertex_ui(device UiVertexStruct *vertexArray [[ buffer(0) ]],
                                    constant UiInstanceStruct &perInstance [[ buffer(1) ]],
                                    unsigned int vid [[ vertex_id ]])
{
    float4 newPosition = perInstance.transformation * vertexArray[vid].position;

    UiFragmentInStruct out;
    out.position = newPosition;
    return out;
}

// remember to use setFragmentBytes for the first buffer
fragment float4 fragment_ui(UiFragmentInStruct in [[stage_in]],
                            constant float4 &color [[ buffer(0) ]])
{
    return color;
}
