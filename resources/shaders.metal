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

// TODO put the height map behaviors in kernels

struct FlatVertex {
    float2 position;
};

struct FlatFragment {
    float4 position [[ position ]];
};

vertex FlatFragment flat_vert(device FlatVertex *vertexArray [[ buffer(0) ]],
                              constant float4x4 &projection [[ buffer(1) ]],
                              constant float4x4 &view [[ buffer(2) ]],
                              texture2d<float, access::read> heightMap [[ texture(0) ]],
                              unsigned int vid [[ vertex_id ]])
{
    float2 pos = vertexArray[vid].position;
    float4 _h = heightMap.read(uint2(pos.x, pos.y));
    float height = (_h.x + _h.y + _h.z) / 3.0;

    FlatFragment out;
    out.position = projection * view * float4(pos.x, -1.0 + height, pos.y, 1.0);
    return out;
};

fragment float4 water_frag(FlatFragment in [[ stage_in ]])
{
    return float4(0.0, 0.5, 1.0, 1.0);
};
