// pathfinder/shaders/gles2/ecaa-cover.vs.glsl
//
// Copyright (c) 2017 Mozilla Foundation

precision highp float;

uniform mat4 uTransform;
uniform ivec2 uFramebufferSize;
uniform ivec2 uBVertexPositionDimensions;
uniform ivec2 uBVertexPathIDDimensions;
uniform sampler2D uBVertexPosition;
uniform sampler2D uBVertexPathID;

attribute vec2 aQuadPosition;
attribute vec4 aUpperPointIndices;
attribute vec4 aLowerPointIndices;

varying vec2 vHorizontalExtents;

void main() {
    // Fetch B-vertex positions.
    // FIXME(pcwalton): This could be slightly optimized to fetch fewer positions.
    ivec4 pointIndices = ivec4(unpackUInt32Attribute(aUpperPointIndices.xy),
                               unpackUInt32Attribute(aUpperPointIndices.zw),
                               unpackUInt32Attribute(aLowerPointIndices.xy),
                               unpackUInt32Attribute(aLowerPointIndices.zw));
    vec2 upperLeftPosition = fetchFloat2Data(uBVertexPosition,
                                             pointIndices.x,
                                             uBVertexPositionDimensions);
    vec2 upperRightPosition = fetchFloat2Data(uBVertexPosition,
                                              pointIndices.y,
                                              uBVertexPositionDimensions);
    vec2 lowerLeftPosition = fetchFloat2Data(uBVertexPosition,
                                             pointIndices.z,
                                             uBVertexPositionDimensions);
    vec2 lowerRightPosition = fetchFloat2Data(uBVertexPosition,
                                              pointIndices.w,
                                              uBVertexPositionDimensions);

    upperLeftPosition = transformVertexPosition(upperLeftPosition, uTransform);
    upperRightPosition = transformVertexPosition(upperRightPosition, uTransform);
    lowerLeftPosition = transformVertexPosition(lowerLeftPosition, uTransform);
    lowerRightPosition = transformVertexPosition(lowerRightPosition, uTransform);

    /*upperLeftPosition = vec2(0.0, 0.0);
    upperRightPosition = vec2(200.0, 0.0);
    lowerLeftPosition = vec2(0.0, 200.0);
    lowerRightPosition = vec2(200.0, 200.0);*/

    vec4 extents = vec4(min(upperLeftPosition.x, lowerLeftPosition.x),
                        min(min(upperLeftPosition.y, upperRightPosition.y),
                            min(lowerLeftPosition.y, lowerRightPosition.y)),
                        max(upperRightPosition.x, lowerRightPosition.x),
                        max(max(upperLeftPosition.y, upperRightPosition.y),
                            max(lowerLeftPosition.y, lowerRightPosition.y)));

    vec4 roundedExtents = vec4(floor(extents.xy), ceil(extents.zw));

    int pathID = fetchUInt16Data(uBVertexPathID, pointIndices.x, uBVertexPathIDDimensions);

    vec2 position = mix(roundedExtents.xy, roundedExtents.zw, (aQuadPosition + 1.0) / 2.0);
    position = convertScreenToClipSpace(position, uFramebufferSize);
    float depth = convertPathIndexToDepthValue(pathID);
    gl_Position = vec4(position, depth, 1.0);

    vHorizontalExtents = roundedExtents.xz;
}
