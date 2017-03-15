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

fs::path Application::scorefile_path;

void Application::ParseArgs(int argc, char *argv[]){
	if(argc > 0) executable_path = argv[0];
	if(argc > 1){
		scorefile_path = argv[1];
		if(!fs::exists(scorefile_path) || !fs::is_regular_file(scorefile_path)){
			scorefile_path.clear();
		}
	}
}

void Application::Init(){
	if(!TryInitDefaultFont()) throw InitError("Default font was not found.");
	
	//set locale
	setlocale(LC_ALL, "");
	//std::locale::global(std::locale(""));
	
	//set up window
	window.create(sf::VideoMode(w, h, bpp), "CrossChronox v0.0.1", sf::Style::Titlebar | sf::Style::Close);
	//window.setSize(sf::Vector2u(w * 2, h * 2));
	window.setKeyRepeatEnabled(false);
	window.setVerticalSyncEnabled(true);
	
	//set up rendertexture
	if(!renderer.create(w, h, true)) throw InitError("Could not create RenderTexture.");
	renderer.setSmooth(true);
	
	//set up SceneManager
	SceneManager::Init();
	
	//set up InputManager
	InputManager::LoadConfig((GetAppdataPath() / "Config/KeyConfig.json").string());
	InputManager::SetMode("Beat");
}

Application::Application(int argc, char *argv[]){ //: qapp(argc, argv){
	ParseArgs(argc, argv);
}

#include <iostream>

int Application::Run(){
	
	//ウインドウが開いている（ゲームループ）
	bool endflag = false;
	while(window.isOpen()){
		sf::Event event;
		while(window.pollEvent(event)){
			//「クローズが要求された」イベント：ウインドウを閉じる
			if(event.type == sf::Event::Closed){
				endflag = true;
			}
			if(event.type == sf::Event::KeyPressed){
				if(event.key.code == sf::Keyboard::Escape){
					endflag = true;
				}
#ifndef NDEBUG //デバッグ時のみ キーが押された、または離されたとき標準出力に表示
				std::cout << "key code " << event.key.code << " was pressed." << std::endl;
			}
			if(event.type == sf::Event::KeyReleased){
				std::cout << "key code " << event.key.code << " was released." << std::endl;
#endif
			}
		}
		
		//update time
		TimeManager::Update();
		
		//update InputManager
		InputManager::Update();
		
		//Scene Update and Draw
		if(SceneManager::Update() == SceneManager::State::FINISH){
			endflag = true;
		}
		
		//終了処理 finalize application
		if(endflag == true){
			SceneManager::Deinit();
			window.close();
			break;
		}
		
		//描画 drawing
		SceneManager::Draw(window);
		
		//描画終わり
		renderer.display();    //バッファ画面をアップデート
		sf::Sprite sprite(renderer.getTexture());  //バッファ画面用のスプライトを作る
		//window.draw(sprite);    //バッファ画面テクスチャの入ったスプライトを画面に描画
		//ちなみにsf::SpriteのPositionの初期値は(0,0)です。
		window.display();   //描画アップデート
		window.clear();
		renderer.clear(sf::Color::Black);  //バッファ画面を黒でクリア
	}
	return 0;
}

void Application::Quit(){
	
}

Application::~Application(){
	Quit();
}

void Application::HandleException(std::exception& e){
#if !defined(_WIN64) && !defined(_WIN32) //if not Windows
	MBox(window.getSystemHandle(), "Error", e.what(), MBOX_OK | MBOX_ICONERROR);
#else //if Windows
	MessageBoxA(nullptr, e.what(), "Error", MB_OK | MB_ICONERROR);
#endif
}
