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
    ushort4 encodedInfo = heightMap.read(uint2(texturedPos));

    ushort r = ((encodedInfo.r >> 8) & 255);
    // ushort g = ((encodedInfo.g >> 8) & 255);
    // ushort b = ((encodedInfo.b >> 8) & 255);
    // ushort a = ((encodedInfo.a >> 8) & 255);

    // TODO do actual stuff here

    WaterFragment out;
    out.position = projection * view * float4(pos.x, -1.0 + r, pos.y, 1.0);
    return out;
};

fragment float4 water_frag(WaterFragment in [[ stage_in ]])
{
    return float4(0.0, 0.5, 1.0, 1.0);
};

// max 4 waves at once (on a given pixel); 1 for R, G, B, and A channels,
// in each channel is also stored the wave's tick - how long has it been here
// [index] [tick]
// propagation bitwise storage: up | 1, down | 2, left | 4, right | 8
kernel void process_water(constant Wave *waves [[ buffer(0) ]],
                          texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                          texture2d<ushort, access::write> newHeightMap [[ texture(1) ]],
                          uint2 gid [[ thread_position_in_grid ]])
{
    ushort4 currentTile = heightMap.read(gid);
    ushort4 above = heightMap.read(uint2(gid.x, gid.y + 1));
    ushort4 below = heightMap.read(uint2(gid.x, gid.y - 1));
    ushort4 left = heightMap.read(uint2(gid.x - 1, gid.y));
    ushort4 right = heightMap.read(uint2(gid.x + 1, gid.y));

    // TODO add propagation in different directions also

    // upwards propagation

    ushort r = ((below.r >> 8) & 255);
    ushort g = ((below.g >> 8) & 255);
    ushort b = ((below.b >> 8) & 255);
    ushort a = ((below.a >> 8) & 255);

    ushort4 newTile = ushort4(currentTile);

    if ((r & 1) == 1) {
        newTile.r |= 256;
    }
    if ((g & 1) == 1) {
        newTile.g |= 256;
    }
    if ((b & 1) == 1) {
        newTile.b |= 256;
    }
    if ((a & 1) == 1) {
        newTile.a |= 256;
    }

    r = ((above.r >> 8) & 255);
    g = ((above.g >> 8) & 255);
    b = ((above.b >> 8) & 255);
    a = ((above.a >> 8) & 255);

    if ((r & 2) == 2) {
        newTile.r |= 512;
    }
    if ((g & 2) == 2) {
        newTile.g |= 512;
    }
    if ((b & 2) == 2) {
        newTile.b |= 512;
    }
    if ((a & 2) == 2) {
        newTile.a |= 512;
    }

    r = ((left.r >> 8) & 255);
    g = ((left.g >> 8) & 255);
    b = ((left.b >> 8) & 255);
    a = ((left.a >> 8) & 255);

    if ((r & 4) == 4) {
        newTile.r |= 1024;
    }
    if ((g & 4) == 4) {
        newTile.g |= 1024;
    }
    if ((b & 4) == 4) {
        newTile.b |= 1024;
    }
    if ((a & 4) == 4) {
        newTile.a |= 1024;
    }

    r = ((right.r >> 8) & 255);
    g = ((right.g >> 8) & 255);
    b = ((right.b >> 8) & 255);
    a = ((right.a >> 8) & 255);

    if ((r & 8) == 8) {
        newTile.r |= 2048;
    }
    if ((g & 8) == 8) {
        newTile.g |= 2048;
    }
    if ((b & 8) == 8) {
        newTile.b |= 2048;
    }
    if ((a & 8) == 8) {
        newTile.a |= 2048;
    }

    newHeightMap.write(newTile, gid);
};
