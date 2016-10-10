//
//  Application.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "Application.hpp"
#include "Path.hpp"

void Application::Init(){
	//GetPaths
	Path::Init();
	
	//If necessary, set locale
	
	//set up window
	window.create(sf::VideoMode(w, h, bpp), "CrossChronox v0.0.1", sf::Style::Titlebar | sf::Style::Close);
	window.setSize(sf::Vector2u(w * 2, h * 2));
	window.setKeyRepeatEnabled(false);
	
	//set up rendertexture
	renderer.create(w, h, true);
	renderer.setSmooth(true);
	
}

void Application::ParseArgs(int argc, const char *argv[]){
	
}

Application::Application(int argc, const char *argv[]){
	ParseArgs(argc, argv);
	Init();
}

int Application::Run(){
	//ウインドウが開いている（ゲームループ）
	while(window.isOpen()){
		sf::Event event;
		while(window.pollEvent(event)){
			//「クローズが要求された」イベント：ウインドウを閉じる
			if(event.type == sf::Event::Closed || (event.type == sf::Event::KeyPressed && event.key.code == sf::Keyboard::Escape)){
				window.close();
			}
		}
		
		window.clear();     //画面をクリア
		renderer.clear(sf::Color::Black);  //バッファ画面を黒でクリア
		
		//Scene Update and Draw
		
		//描画終わり
		renderer.display();    //バッファ画面をアップデート
		sf::Sprite sprite(renderer.getTexture());  //バッファ画面用のスプライトを作る
		window.draw(sprite);    //バッファ画面テクスチャの入ったスプライトを画面に描画
		//ちなみにsf::SpriteのPositionの初期値は(0,0)です。
		window.display();   //描画アップデート
	}
	return 0;
}

void Application::Quit(){
	
}

Application::~Application(){
	Quit();
}

void Application::HandleException(std::exception e){
	
}
