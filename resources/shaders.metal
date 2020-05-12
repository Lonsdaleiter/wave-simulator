#include <metal_stdlib>

using namespace metal;

struct StaticVertex {
    packed_float3 position;
};

struct StaticFragment {
    float4 position [[ position ]];
};

vertex StaticFragment static_vert(device StaticVertex *vertexArray [[ buffer(0) ]],
                                  constant float4x4 &projection [[ buffer(1) ]],
                                  constant float4x4 &view [[ buffer(2) ]],
                                  constant float4x4 &transformation [[ buffer(3) ]],
                                  uint vid [[ vertex_id ]])
{
    StaticFragment frag;
    frag.position = projection * view * transformation * float4(vertexArray[vid].position, 1.0);
    return frag;
}

fragment float4 static_frag(StaticFragment in [[ stage_in ]],
                            constant float4 &colour [[ buffer(1) ]])
{
    return float4(1.0, 0.0, 0.0, 1.0);
}

struct UiVertex {
	float2 position;
	float2 textureCoords;
};

struct UiTransformation {
    float2 centre;
    float2 size;
};

struct UiFragment {
    float4 position [[ position ]];
    float2 textureCoords;
};

vertex UiFragment ui_vert(device UiVertex *vertexArray [[ buffer(0) ]],
                          constant UiTransformation &transformation [[ buffer(1) ]],
                          uint vid [[ vertex_id ]])
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
    return texture.sample(sam, in.textureCoords);
}

struct WaterVertex {
    float2 position;
};

struct WaterFragment {
    float4 position [[ position ]];
    float realHeight;
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
                                uint vid [[ vertex_id ]])
{
    float2 pos = vertexArray[vid].position;

    int2 texturedPos = int2(pos);
    texturedPos += 50;
    texturedPos.y = 100 - texturedPos.y;
    ushort4 encodedInfo = heightMap.read(uint2(texturedPos));
    ushort4 tickPos = encodedInfo & 255;
    ushort4 waveBoolChart = (encodedInfo >> 8) & 255;
    float r = 0;
    float g = 0;
    float b = 0;
    float a = 0;
    if (waveBoolChart.r == 1) {
        r = waves[0].amplitude * sin(float(tickPos.r) * (M_PI_F / float(waves[0].wavelength)));
    }
    if (waveBoolChart.g == 1) {
        g = waves[1].amplitude * sin(float(tickPos.g) * (M_PI_F / float(waves[1].wavelength)));
    }
    if (waveBoolChart.b == 1) {
        b = waves[2].amplitude * sin(float(tickPos.b) * (M_PI_F / float(waves[2].wavelength)));
    }
    if (waveBoolChart.a == 1) {
        a = waves[3].amplitude * sin(float(tickPos.a) * (M_PI_F / float(waves[3].wavelength)));
    }

    float amplitude = r + g + b + a;
    float4 finalPosition = float4(pos.x, amplitude, pos.y, 1.0);

    WaterFragment out;
    out.position = projection * view * finalPosition;
    out.realHeight = finalPosition.y;
    return out;
};

fragment float4 water_frag(WaterFragment in [[ stage_in ]])
{
    return float4(abs(in.realHeight), 0.7, 1.0, 1.0);
};

// max 4 waves at once (on a given pixel); 1 for R, G, B, and A channels,
// it is UNDEFINED BEHAVIOR to have a channel have a value other than 0 or 1 in the first byte
// in each channel is also stored the wave's tick - how long has it been here
// [index] [tick]; ticks should NEVER overflow into indices (wavelength < 256) and there should
// be checks to stop ticks and toggle the boolean whenever ticks hit wavelength cap
// propagation bitwise storage: up | 1, down | 2, left | 4, right | 8
kernel void process_water(constant Wave *waves [[ buffer(0) ]],
                          texture2d<ushort, access::read> heightMap [[ texture(0) ]],
                          texture2d<ushort, access::write> newHeightMap [[ texture(1) ]],
                          uint2 gid [[ thread_position_in_grid ]])
{
    ushort4 currentTile = heightMap.read(gid);

    ushort4 above = heightMap.read(uint2(gid.x, gid.y + 1));

    if (((currentTile.r >> 8) & 255) != 1 && ((above.r >> 8) & 255) == 1 && (waves[0].directions & 1) == 1) {
        currentTile.r = 256;
    }
    if (((currentTile.g >> 8) & 255) != 1 && ((above.g >> 8) & 255) == 1 && (waves[1].directions & 1) == 1) {
        currentTile.g = 256;
    }
    if (((currentTile.b >> 8) & 255) != 1 && ((above.b >> 8) & 255) == 1 && (waves[2].directions & 1) == 1) {
        currentTile.b = 256;
    }
    if (((currentTile.a >> 8) & 255) != 1 && ((above.a >> 8) & 255) == 1 && (waves[3].directions & 1) == 1) {
        currentTile.a = 256;
    }

    ushort4 below = heightMap.read(uint2(gid.x, gid.y - 1));
    
    if (((currentTile.r >> 8) & 255) != 1 && ((below.r >> 8) & 255) == 1 && (waves[0].directions & 2) == 2) {
        currentTile.r = 256;
    }
    if (((currentTile.g >> 8) & 255) != 1 && ((below.g >> 8) & 255) == 1 && (waves[1].directions & 2) == 2) {
        currentTile.g = 256;
    }
    if (((currentTile.b >> 8) & 255) != 1 && ((below.b >> 8) & 255) == 1 && (waves[2].directions & 2) == 2) {
        currentTile.b = 256;
    }
    if (((currentTile.a >> 8) & 255) != 1 && ((below.a >> 8) & 255) == 1 && (waves[3].directions & 2) == 2) {
        currentTile.a = 256;
    }
    
    ushort4 left = heightMap.read(uint2(gid.x - 1, gid.y));
    
    if (((currentTile.r >> 8) & 255) != 1 && ((left.r >> 8) & 255) == 1 && (waves[0].directions & 4) == 4) {
        currentTile.r = 256;
    }
    if (((currentTile.g >> 8) & 255) != 1 && ((left.g >> 8) & 255) == 1 && (waves[1].directions & 4) == 4) {
        currentTile.g = 256;
    }
    if (((currentTile.b >> 8) & 255) != 1 && ((left.b >> 8) & 255) == 1 && (waves[2].directions & 4) == 4) {
        currentTile.b = 256;
    }
    if (((currentTile.a >> 8) & 255) != 1 && ((left.a >> 8) & 255) == 1 && (waves[3].directions & 4) == 4) {
        currentTile.a = 256;
    }
    
    ushort4 right = heightMap.read(uint2(gid.x + 1, gid.y));

    if (((currentTile.r >> 8) & 255) != 1 && ((right.r >> 8) & 255) == 1 && (waves[0].directions & 8) == 8) {
        currentTile.r = 256;
    }
    if (((currentTile.g >> 8) & 255) != 1 && ((right.g >> 8) & 255) == 1 && (waves[1].directions & 8) == 8) {
        currentTile.g = 256;
    }
    if (((currentTile.b >> 8) & 255) != 1 && ((right.b >> 8) & 255) == 1 && (waves[2].directions & 8) == 8) {
        currentTile.b = 256;
    }
    if (((currentTile.a >> 8) & 255) != 1 && ((right.a >> 8) & 255) == 1 && (waves[3].directions & 8) == 8) {
        currentTile.a = 256;
    }

    if (((currentTile.r >> 8) & 255) == 1) {
        if ((currentTile.r & 255) < waves[0].wavelength) {
            currentTile.r += 1;
        } else {
            currentTile.r = 0;
        }
    }
    if (((currentTile.g >> 8) & 255) == 1) {
        if ((currentTile.g & 255) < waves[1].wavelength) {
            currentTile.g += 1;
        } else {
            currentTile.g = 0;
        }
    }
    if (((currentTile.b >> 8) & 255) == 1) {
        if ((currentTile.b & 255) < waves[2].wavelength) {
            currentTile.b += 1;
        } else {
            currentTile.b = 0;
        }
    }
    if (((currentTile.a >> 8) & 255) == 1) {
        if ((currentTile.a & 255) < waves[3].wavelength) {
            currentTile.a += 1;
        } else {
            currentTile.a = 0;
        }
    }

    newHeightMap.write(currentTile, gid);
};
