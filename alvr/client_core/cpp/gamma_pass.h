#pragma once

#include <cstdint>
#include <memory>
#include <vector>
#include "gl_render_utils/render_pipeline.h"

class GammaPass {
public:
    GammaPass(gl_render_utils::Texture *inputSurface);

    void Initialize(uint32_t width, uint32_t height);

    void Render() const;

    gl_render_utils::Texture *GetOutputTexture() { return mOutputTexture.get(); }

private:

    gl_render_utils::Texture *mInputSurface;
    std::unique_ptr<gl_render_utils::Texture> mOutputTexture;
    std::unique_ptr<gl_render_utils::RenderState> mOutputTextureState;
    std::unique_ptr<gl_render_utils::RenderPipeline> mStagingPipeline;
};
