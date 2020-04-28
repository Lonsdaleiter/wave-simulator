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
    float2 position;
};

struct WaterFragment {
    float4 position [[ position ]];
};

struct Wave {
    char directions;
    char wavelength;
    float amplitude;
};

vertex WaterFragment water_vert(device WaterVertex *vertexArray [[ buffer(0) ]],
                                constant float4x4 &projection [[ buffer(1) ]],
                                constant float4x4 &view [[ buffer(2) ]],
                                constant Wave *waves [[ buffer(3) ]],
                                texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                                unsigned int vid [[ vertex_id ]])
{
    float2 pos = vertexArray[vid].position;

    int2 texturedPos = int2(pos);
    texturedPos += 50;
    texturedPos.y = 100 - texturedPos.y;
    // ushort4 encodedInfo = heightMap.read(uint2(texturedPos));

    WaterFragment out;
    out.position = projection * view * float4(pos.x, -1.0, pos.y, 1.0);
    return out;
};

fragment float4 water_frag(WaterFragment in [[ stage_in ]])
{
    return float4(0.0, 0.5, 1.0, 1.0);
};

// max 4 waves at once (on a given pixel); 1 for R, G, B, and A channels,
// it is UNDEFINED BEHAVIOR to have a channel have a value other than 0 or 1 in the first byte
// in each channel is also stored the wave's tick - how long has it been here
// [index] [tick]
// propagation bitwise storage: up | 1, down | 2, left | 4, right | 8
kernel void process_water(constant Wave *waves [[ buffer(0) ]],
                          texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                          texture2d<ushort, access::write> newHeightMap [[ texture(1) ]],
                          uint2 gid [[ thread_position_in_grid ]])
{
    // ushort4 currentTile = heightMap.read(gid);
    // ushort4 above = heightMap.read(uint2(gid.x, gid.y + 1));
    // ushort4 below = heightMap.read(uint2(gid.x, gid.y - 1));
    // ushort4 left = heightMap.read(uint2(gid.x - 1, gid.y));
    // ushort4 right = heightMap.read(uint2(gid.x + 1, gid.y));

    newHeightMap.write(ushort4(0, 0, 0, 0), gid);
};
