//
//  SceneManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/24/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "SceneManager.hpp"
#include "PlayScore.hpp"
#include "SelectMusic.hpp"

namespace SceneManager{
	Scene* now_scene = scene_select_music_ptr;
	
	void Init(){
		now_scene->Init();
	}
	
	State Update(){
		Scene* next_scene = now_scene->Update();
		if(now_scene != next_scene){
			now_scene->Deinit();
			if(next_scene == nullptr){
				return FINISH;
			}
			now_scene = next_scene;
			next_scene->Init();
			next_scene->Update();
		}
		return CONTINUE;
	}
	
	void Draw(SDL_Renderer* renderer){
		if(now_scene) now_scene->Draw(renderer);
	}
	
	void Deinit(){
		if(now_scene){
			now_scene->Deinit();
			now_scene = nullptr;
		}
	}
}
