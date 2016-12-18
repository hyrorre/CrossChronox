//
//  Application.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "Application.hpp"
#include "Path.hpp"
#include "ScorePlayer.hpp"
#include "BmsLoader.hpp"
#include "TimeManager.hpp"
#include "SceneManager.hpp"

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
	//GetPaths
	Path::Init();
	
	//If necessary, set locale
	
	//set up window
	window.create(sf::VideoMode(w, h, bpp), "CrossChronox v0.0.1", sf::Style::Titlebar | sf::Style::Close);
	window.setSize(sf::Vector2u(w * 2, h * 2));
	window.setKeyRepeatEnabled(false);
	
	//set up rendertexture
	if(!renderer.create(w, h, true)) throw InitError("Could not create RenderTexture.");
	renderer.setSmooth(true);
	
	//for testing
	//if file was not found, this will be ignored.
	scorefile_path = "/Volumes/Attached/BMS/[xi] Halcyon/_hal_A.bml";
	
	//set up SceneManager
	SceneManager::Init();
}

Application::Application(int argc, char *argv[]): qapp(argc, argv){
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
			}
		}
		
		//update time
		TimeManager::Update();
		
		//Scene Update and Draw
		if(SceneManager::Update() == SceneManager::State::FINISH){
			SceneManager::Deinit();
			endflag = true;
		}
		
		//終了処理 finalize application
		if(endflag == true){
			SceneManager::Deinit();
			window.close();
			break;
		}
		
		//描画終わり
		renderer.display();    //バッファ画面をアップデート
		sf::Sprite sprite(renderer.getTexture());  //バッファ画面用のスプライトを作る
		window.draw(sprite);    //バッファ画面テクスチャの入ったスプライトを画面に描画
		//ちなみにsf::SpriteのPositionの初期値は(0,0)です。
		window.display();   //描画アップデート
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
    QMessageBox msgBox;
	QTextCodec* tc = QTextCodec::codecForLocale();
	msgBox.setText(tc->toUnicode(e.what()));
	msgBox.setWindowTitle(tc->toUnicode("Error")); //this is ignored on macOS
	msgBox.setIcon(QMessageBox::Icon::Critical);
	msgBox.exec();
}
