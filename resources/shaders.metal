#include <metal_stdlib>

using namespace metal;

struct UiVertexStruct {
	float4 position;
};

struct UiFragmentInStruct {
    float4 position [[position]];
};

vertex UiFragmentInStruct vertex_ui(device UiVertexStruct *vertexArray [[ buffer(0) ]],
                                    unsigned int vid [[ vertex_id ]])
{
    UiFragmentInStruct out;
    out.position = vertexArray[vid].position;
    return out;
}

// remember to use setFragmentBytes for the color variable
fragment float4 fragment_ui(UiFragmentInStruct in [[stage_in]],
                            constant float4 &color [[ buffer(0) ]])
{
    //return float4(0.0, 1.0, 1.0, 1.0);
    return color;
}
