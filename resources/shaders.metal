#include <metal_stdlib>

using namespace metal;

struct UiVertex {
	float2 position;
	float2 textureCoords;
};

struct UiTransformation {
    float2 centre;
    float2 size;
};

struct UiFragment {
    float4 position [[position]];
    float2 textureCoords;
};

vertex UiFragment ui_vert(device UiVertex *vertexArray [[ buffer(0) ]],
                          constant UiTransformation &transformation [[ buffer(1) ]],
                          unsigned int vid [[ vertex_id ]])
{
    UiFragment frag;
    frag.position = float4(vertexArray[vid].position * transformation.size + transformation.centre, 0.0, 1.0);
    frag.textureCoords = vertexArray[vid].textureCoords;
    return frag;
}

fragment float4 ui_frag(UiFragment in [[stage_in]],
                        texture2d<float> texture [[ texture(0) ]],
                        sampler sam [[ sampler(0) ]])
{
    return float4(0.0, 1.0, 1.0, 1.0);
}

struct WaterVertex {
    packed_float3 position;
};

struct WaterFragment {
    float4 position [[ position ]];
};

vertex WaterFragment water_vert(device WaterVertex *vertexArray [[ buffer(0) ]],
                                constant float4x4 &projection [[ buffer(1) ]],
                                constant float4x4 &view [[ buffer(2) ]],
                                unsigned int vid [[ vertex_id ]])
{
    float3 position = vertexArray[vid].position;
    position.z = -position.z;
    position.y -= 1;

    WaterFragment out;
    out.position = projection * view * float4(position, 1.0);
    return out;
};

fragment float4 water_frag(WaterFragment in [[ stage_in ]])
{
    return float4(0.0, 0.0, 1.0, 1.0);
};
