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

struct FlatVertex {
    float2 position;
};

struct FlatFragment {
    float4 position [[ position ]];
};

vertex FlatFragment flat_vert(device FlatVertex *vertexArray [[ buffer(0) ]],
                              constant float4x4 &projection [[ buffer(1) ]],
                              constant float4x4 &view [[ buffer(2) ]],
                              texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                              unsigned int vid [[ vertex_id ]])
{
    float2 pos = vertexArray[vid].position;
    float height = heightMap.read(uint2(pos.x, pos.y)).r;

    FlatFragment out;
    out.position = projection * view * float4(pos.x, -1.0 + float(height) / 1000.0, pos.y, 1.0);
    return out;
};

fragment float4 water_frag(FlatFragment in [[ stage_in ]])
{
    return float4(0.0, 0.5, 1.0, 1.0);
};

// determine the height by the red
// determine the propagation by the green
// blue and alpha are reserved for future use
// note that newHeightMap is heightMap
kernel void process_water(texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                          texture2d<ushort, access::write> newHeightMap [[ texture(1) ]],
                          uint2 gid [[ thread_position_in_grid ]])
{
    ushort4 height = heightMap.read(gid);
    ushort4 above = heightMap.read(uint2(gid.x, gid.y + 1));
    ushort4 below = heightMap.read(uint2(gid.x, gid.y - 1));
    ushort4 left = heightMap.read(uint2(gid.x - 1, gid.y));
    ushort4 right = heightMap.read(uint2(gid.x + 1, gid.y));

    ushort4 newColour = ushort4(height.r, 0.0, 0.0, 0.0);

    if ((above.g & 1) == 1) {
        newColour.g = newColour.g | 1;
    }
    if ((below.g & 2) == 1) {
        newColour.g = newColour.g | 2;
    }
    if ((left.g & 4) == 1) {
        newColour.g = newColour.g | 4;
    }
    if ((right.g & 8) == 1) {
        newColour.g = newColour.g | 8;
    }

    newHeightMap.write(newColour, gid);
};
