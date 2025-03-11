﻿#pragma once

#include "pch.hpp"

class SubScene;

class Scene {
  public:
    Scene() {}
    virtual ~Scene() {}

    virtual void Init() {}   // called when scene switched in
    virtual void Deinit() {} // called when scene switched out

    virtual Scene* Update() = 0;                                  // update data
    virtual void Draw(sf::RenderTarget& render_target) = 0; // シーンの描画
};
