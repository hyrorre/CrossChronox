//
//  SceneManager.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/24/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef SceneManager_hpp
#define SceneManager_hpp

#include "pch.hpp"

class IScene{
public:
	IScene();
	virtual ~IScene();
	virtual void Update() = 0; // Update Data
	virtual void Render() = 0; // シーンの描画
};

class SceneManager{
	using scene_stack_type = std::stack<std::vector<std::unique_ptr<IScene>>>;
	scene_stack_type scenes;
	
public:
	//argument is raw pointer (not smart pointer)
	static void Push(IScene* new_scene);
	static void Pop();
	//static IScene* top();
	//static const IScene* ctop();
	static void Clear();
};

#endif /* SceneManager_hpp */
