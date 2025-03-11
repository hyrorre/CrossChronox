#include "Application.hpp"
#include "Filesystem/Path.hpp"
#include "Platform/MessageBox.hpp"
#include "Scene/SceneManager.hpp"
#include "Score/Load/BmsLoader.hpp"
#include "Score/Play/ScorePlayer.hpp"
#include "System/Input/InputManager.hpp"
#include "System/Setting.hpp"
#include "System/TimeManager.hpp"

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
    if (!TryInitDefaultFont())
        throw InitError("Default font was not found.");

    // set locale
    setlocale(LC_ALL, "");
    // std::locale::global(std::locale(""));

    setting.TryLoadFile((GetAppdataPath() / "Config/Setting.toml").string());

    int w = setting.GetResolutionX();
    int h = setting.GetResolutionY();
    int bpp = 32;

    auto style = sf::Style::Close | sf::Style::Titlebar;
    if (setting.GetWindowType() == FULLSCREEN)
        style = sf::Style::Fullscreen;

    // set up window
    window.create(sf::VideoMode(w, h, bpp), "CrossChronox v0.0.1", style);
    window.setSize(sf::Vector2u(setting.GetWindowSizeX(), setting.GetWindowSizeY()));
    window.setKeyRepeatEnabled(false);
    window.setVerticalSyncEnabled(setting.GetVsync());
    if (unsigned limit = setting.GetMaxFps()) { // max_fps == 0 のとき無制限
        window.setFramerateLimit(limit);
    }
    window.setPosition(setting.GetWindowPos());

    // set up rendertexture
    if (!renderer.create(w, h, true))
        throw InitError("Could not create RenderTexture.");
    renderer.setSmooth(true);

    // set up SceneManager
    scene_manager.Init();

    // set up InputManager
    InputManager::LoadConfig((GetAppdataPath() / "Config/KeyConfig.toml").string());
    InputManager::SetMode("Beat");
}

Application::Application(int argc, char* argv[]) {
    ParseArgs(argc, argv);
}

int Application::Run() {

    // ウインドウが開いている（ゲームループ）
    bool endflag = false;
    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            // 「クローズが要求された」イベント：ウインドウを閉じる
            if (event.type == sf::Event::Closed) {
                endflag = true;
            }
        }

        // update time
        TimeManager::Update();

        // update InputManager
        InputManager::Update();

        // Scene Update and Draw
        if (scene_manager.Update() == SceneManager::State::FINISH) {
            endflag = true;
        }

        // 終了処理 finalize application
        if (endflag == true) {
            scene_manager.Deinit();
            window.close();
            break;
        }

        // 描画 drawing
        scene_manager.Draw(window);

        // 描画終わり
        renderer.display();                       // バッファ画面をアップデート
        sf::Sprite sprite(renderer.getTexture()); // バッファ画面用のスプライトを作る
        // window.draw(sprite);    //バッファ画面テクスチャの入ったスプライトを画面に描画
        // ちなみにsf::SpriteのPositionの初期値は(0,0)です。
        window.display(); // 描画アップデート
        window.clear();
        renderer.clear(sf::Color::Black); // バッファ画面を黒でクリア
    }
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
    MessageBoxA(window.getSystemHandle(), e.what(), "Error", MB_OK | MB_ICONERROR);
}
