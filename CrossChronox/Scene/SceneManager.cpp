//
//  SceneManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/24/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "SceneManager.hpp"
#include "PlayScore.hpp"

namespace SceneManager{
	Scene* now_scene = scene_play_score_ptr;
	
	State Update(){
		Scene* next_scene = now_scene->Update();
		if(now_scene != next_scene){
			if(next_scene == nullptr){
				return FINISH;
			}
			now_scene = next_scene;
			now_scene->Init();
			now_scene->Update();
		}
		return CONTINUE;
	}
	
	void Draw(){
		now_scene->Draw();
	}
}
