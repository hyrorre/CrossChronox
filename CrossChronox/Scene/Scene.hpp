//
//  Scene.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/17.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef Scene_hpp
#define Scene_hpp

#include "pch.hpp"

class SubScene;

class Scene{
public:
	Scene(){}
	virtual ~Scene(){}
	
//	using scene_stack_t = std::stack<std::unique_ptr<SubScene>, std::vector<std::unique_ptr<SubScene>>>;
//	scene_stack_t sub_scenes;
	
	virtual void Init(){}            // called when scene switched in
	virtual void Deinit(){}          // called when scene switched out
	
	virtual Scene* Update() = 0;     // update data
	virtual void Draw(SDL_Renderer* renderer) const = 0;   // シーンの描画
};

#endif /* Scene_hpp */
