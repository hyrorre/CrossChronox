#pragma once

#include "pch.hpp"

class SubScene;

class Scene {
  public:
    Scene() {}
    virtual ~Scene() {}

    //	using scene_stack_t = std::stack<std::unique_ptr<SubScene>, std::vector<std::unique_ptr<SubScene>>>;
    //	scene_stack_t sub_scenes;

    virtual void Init() {}   // called when scene switched in
    virtual void Deinit() {} // called when scene switched out

    virtual Scene* Update() = 0;                                  // update data
    virtual void Draw(sf::RenderTarget& render_target) const = 0; // シーンの描画
};
