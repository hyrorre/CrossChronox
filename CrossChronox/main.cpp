#include "pch.hpp"
#include "FileSystem/Path.hpp"

int main() {
    // 画面サイズ
    const int screenWidth = 1920;
    const int screenHeight = 1080;

    // ウィンドウの作成
    InitWindow(screenWidth, screenHeight, "raylib - PNG Load Example");

    // テクスチャのロード (PNG 画像)
    Texture2D texture = LoadTexture((GetAppdataPath() / "play.png").string().c_str());

    int i = 0;
    SetTargetFPS(60);

    // メインループ
    while (!WindowShouldClose()) { // ESCキーかウィンドウを閉じたら終了
        BeginDrawing();
        ClearBackground(BLACK);

        // 画像の描画 (左上に配置)
        DrawTexture(texture, ++i, 0, WHITE);

        DrawText("PNG Image Displayed!", 200, 400, 20, WHITE);

        EndDrawing();
    }

    // メモリ解放
    UnloadTexture(texture);
    CloseWindow();

    return 0;
}
