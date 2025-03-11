﻿#include "Application.hpp"
#include "Filesystem/Path.hpp"
#include "Platform/MessageBox.hpp"
#include "Scene/SceneManager.hpp"
#include "Score/Load/BmsLoader.hpp"
#include "Score/Play/ScorePlayer.hpp"
#include "System/DefaultFont.hpp"
#include "System/Input/InputManager.hpp"
#include "System/Setting.hpp"
#include "System/TimeManager.hpp"

fs::path Application::scorefile_path;

void Application::ParseArgs(int argc, char* argv[]) {
    if (argc > 0)
        executable_path = argv[0];
    if (argc > 1) {
        scorefile_path = argv[1];
        if (!fs::exists(scorefile_path) || !fs::is_regular_file(scorefile_path)) {
            scorefile_path.clear();
        }
    }
}

void Application::Init() {
    // set locale
    setlocale(LC_ALL, "");
    // std::locale::global(std::locale(""));

    setting.TryLoadFile((GetAppdataPath() / "Config/Setting.toml").string());

    unsigned int w = setting.GetResolutionX();
    unsigned int h = setting.GetResolutionY();
    int bpp = 32;

    auto style = sf::Style::Close | sf::Style::Titlebar;
    sf::State state = setting.GetWindowType() == FULLSCREEN ? sf::State::Fullscreen : sf::State::Windowed;

    // set up window
    window.create(sf::VideoMode({w, h}, bpp), "CrossChronox v0.0.1", style);
    window.setSize(sf::Vector2u(setting.GetWindowSizeX(), setting.GetWindowSizeY()));
    window.setKeyRepeatEnabled(false);
    window.setVerticalSyncEnabled(setting.GetVsync());
    if (unsigned limit = setting.GetMaxFps()) { // max_fps == 0 のとき無制限
        window.setFramerateLimit(limit);
    }
    window.setPosition(setting.GetWindowPos());

    // set up rendertexture
    if (!renderer.resize({w, h}))
        throw InitError("Could not create RenderTexture.");
    renderer.setSmooth(true);

    // set up SceneManager
    SceneManager::Init();

    // set up InputManager
    InputManager::LoadConfig((GetAppdataPath() / "Config/KeyConfig.toml").string());
    InputManager::SetMode("Beat");
}

Application::Application(int argc, char* argv[]) {
    ParseArgs(argc, argv);
}

std::atomic<bool> endflag(false);

void Application::Update() {
    window.setActive(true);

    while (!endflag) {
        // update time
        TimeManager::Update();

        // update InputManager
        InputManager::Update();

        // Scene Update and Draw
        if (SceneManager::Update() == SceneManager::State::FINISH) {
            endflag = true;
        }

        // 描画 drawing
        SceneManager::Draw(window);

        // 描画終わり
        renderer.display();                       // バッファ画面をアップデート
        sf::Sprite sprite(renderer.getTexture()); // バッファ画面用のスプライトを作る
        // window.draw(sprite);    //バッファ画面テクスチャの入ったスプライトを画面に描画
        // ちなみにsf::SpriteのPositionの初期値は(0,0)です。
        window.display(); // 描画アップデート
        window.clear();
        renderer.clear(sf::Color::Black); // バッファ画面を黒でクリア
    }

    // 終了処理 finalize application
    SceneManager::Deinit();
    window.close();
}

int Application::Run() {
    // start new thread for main game process
    // メインスレッド以外でレンダリングしないと、ウィンドウのドラッグ中に処理が止まってしまう
    window.setActive(false);
    std::thread updateThreadInstance(&Application::Update, this);

    // only handling close event in main thread
    while (window.isOpen()) {
        while (const std::optional event = window.pollEvent()) {
            // 「クローズが要求された」イベント：ウインドウを閉じる
            if (event->is<sf::Event::Closed>()) {
                endflag = true;
            }
        }

        sf::sleep(sf::milliseconds(1));
    }
    updateThreadInstance.join();
    return 0;
}

void Application::Quit() {
    // TODO: なぜかgetPosition()が絶対に(0, 0)を返すため保留
    // setting.SetWindowPos(window.getPosition());
    setting.SaveFile((GetAppdataPath() / "Config/Setting.toml").string());
}

Application::~Application() {
    Quit();
}

void Application::HandleException(std::exception& e) {
    MessageBoxA(window.getNativeHandle(), e.what(), "Error", MB_OK | MB_ICONERROR);
}
