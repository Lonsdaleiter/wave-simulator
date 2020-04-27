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
    float height;
};

struct Wave {
    char directions;
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
    // TODO do stuff involving the current waves for amplitude; don't get height from pixel
    float height = heightMap.read(uint2((pos.x + 50.0) / 2.0, (pos.y + 50.0) / 2.0)).r;

    WaterFragment out;
    out.position = projection * view * float4(pos.x, -1.0 + float(height) / 1000.0, pos.y, 1.0);
    out.height = float(height);
    return out;
};

fragment float4 water_frag(WaterFragment in [[ stage_in ]])
{
    return float4(in.height, 0.5, 1.0, 1.0);
};

// TODO redo this compute kernel to encode in the texture data pointing to a constant array of wave structs containing the data
// max of 255 types of waves total, max 6 waves at once (on a given pixel)
kernel void process_water(constant Wave *waves [[ buffer(0) ]],
                          texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                          texture2d<ushort, access::write> newHeightMap [[ texture(1) ]],
                          uint2 gid [[ thread_position_in_grid ]])
{
    ushort4 height = heightMap.read(gid);
    ushort4 above = heightMap.read(uint2(gid.x, gid.y + 1));
    ushort4 below = heightMap.read(uint2(gid.x, gid.y - 1));
    ushort4 left = heightMap.read(uint2(gid.x - 1, gid.y));
    ushort4 right = heightMap.read(uint2(gid.x + 1, gid.y));

    ushort4 newColour = ushort4(height.r, 0, 0, 0);

    if ((above.g & 1) == 1) {
        newColour.r = 1000.0;
        // waves segregated
        newColour.g = newColour.g | above.g;
    }
    if ((below.g & 2) == 2) {
        newColour.r = 1000.0;
        newColour.g = newColour.g | below.g;
    }
    if ((left.g & 4) == 4) {
        newColour.r = 1000.0;
        newColour.g = newColour.g | left.g;
    }
    if ((right.g & 8) == 8) {
        newColour.r = 1000.0;
        newColour.g = newColour.g | right.g;
    }

    newHeightMap.write(newColour, gid);
};
