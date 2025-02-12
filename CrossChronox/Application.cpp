﻿//
//  Application.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "Application.hpp"
#include "Filesystem/Path.hpp"
#include "Score/Play/ScorePlayer.hpp"
#include "Score/Load/BmsLoader.hpp"
#include "System/TimeManager.hpp"
#include "Scene/SceneManager.hpp"
#include "System/Input/InputManager.hpp"
#include "System/DefaultFont.hpp"
#include "Platform/MessageBox.hpp"
#include "System/Setting.hpp"
//
Application app;
fs::path Application::scorefile_path;
//
//void Application::ParseArgs(int argc, char *argv[]){
//	if(argc > 0) executable_path = argv[0];
//	if(argc > 1){
//		scorefile_path = argv[1];
//		if(!fs::exists(scorefile_path) || !fs::is_regular_file(scorefile_path)){
//			scorefile_path.clear();
//		}
//	}
//}
//
SDL_AppResult Application::Init(){
	//set locale
	setlocale(LC_ALL, "");

    if (!SDL_Init(SDL_INIT_VIDEO)) {
        SDL_Log("Couldn't initialize SDL: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }

    if (!SDL_CreateWindowAndRenderer("CrossChronox", 800, 600, 0, &window, &renderer)) {
        SDL_Log("Couldn't create window/renderer: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }

    InputManager::LoadConfig((GetAppdataPath() / "Config/KeyConfig.json").string());
	InputManager::SetMode("Beat");

    return SDL_APP_CONTINUE;
}

SDL_AppResult Application::Event(SDL_Event* event) {
    if (event->type == SDL_EVENT_QUIT) {
        return SDL_APP_SUCCESS;  /* end the program, reporting success to the OS. */
    }
    return SDL_APP_CONTINUE;  /* carry on with the program! */
}


SDL_AppResult Application::Run() {
    //update time
    TimeManager::Update();

    //update InputManager
    InputManager::Update();

    //描画 drawing
    SceneManager::Draw(renderer);

    //描画終わり
    SDL_RenderPresent(renderer);    //バッファ画面をアップデート
    return SDL_APP_CONTINUE;
}
//		//update time
//		TimeManager::Update();
//		
//		//update InputManager
//		InputManager::Update();
//		
//		//Scene Update and Draw
//		if(SceneManager::Update() == SceneManager::State::FINISH){
//			endflag = true;
//		}
//		
//		//終了処理 finalize application
//		if(endflag == true){
//			SceneManager::Deinit();
//			window.close();
//			break;
//		}
//		
//		//描画 drawing
//		SceneManager::Draw(window);
//		
//		//描画終わり
//		renderer.display();    //バッファ画面をアップデート
//		sf::Sprite sprite(renderer.getTexture());  //バッファ画面用のスプライトを作る
//		//window.draw(sprite);    //バッファ画面テクスチャの入ったスプライトを画面に描画
//		//ちなみにsf::SpriteのPositionの初期値は(0,0)です。
//		window.display();   //描画アップデート
//		window.clear();
//		renderer.clear(sf::Color::Black);  //バッファ画面を黒でクリア
//	}
//	return 0;
//

void Application::Quit(){
}
