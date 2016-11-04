//
//  Application.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "Application.hpp"
#include "Path.hpp"
#include "BmsLoader.hpp"
#include "TimeManager.hpp"

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
}

Application::Application(int argc, char *argv[]): qapp(argc, argv){
	ParseArgs(argc, argv);
}

int Application::Run(){
	ScoreData score;
	
	if(!scorefile_path.empty()){
		bms::Load(scorefile_path.string(), &score);
	}
	
	
	//ウインドウが開いている（ゲームループ）
	while(window.isOpen()){
		sf::Event event;
		while(window.pollEvent(event)){
			//「クローズが要求された」イベント：ウインドウを閉じる
			if(event.type == sf::Event::Closed){
				window.close();
			}
			if(event.type == sf::Event::KeyPressed){
				if(event.key.code == sf::Keyboard::Escape){
					window.close();
				}
			}
		}
		
		//update time
		TimeManager::Update();
		
		//Scene Update and Draw
		
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
