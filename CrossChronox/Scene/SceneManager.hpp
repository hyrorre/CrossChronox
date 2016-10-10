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
public:
	//argument is raw pointer (not smart pointer)
	static void push(IScene* new_scene);
	static void pop();
	//static IScene* top();
	//static const IScene* ctop();
	static void clear();
};

#endif /* SceneManager_hpp */
