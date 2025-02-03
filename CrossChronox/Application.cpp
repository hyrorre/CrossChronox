#include <filesystem>
#include <exception>
#include <stdexcept>

#define SDL_MAIN_USE_CALLBACKS 1  /* use the callbacks instead of main() */
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <SDL3_image/SDL_image.h>

static SDL_Window* window = nullptr;
static SDL_Renderer* renderer = nullptr;
static SDL_Texture* texture = nullptr;

const std::filesystem::path& GetDataPath() {
    static std::filesystem::path data_path;
    if (data_path.empty()) {
        data_path = std::filesystem::current_path();
        while (data_path.has_parent_path()) {
            std::filesystem::path tmp_path = data_path / "CrossChronoxData";
            if (std::filesystem::exists(tmp_path)) {
                data_path = tmp_path;
                return data_path;
            }
            data_path = data_path.parent_path();
        }
        throw std::runtime_error("\"CrossChronoxData\" folder was not found.");
    }
    return data_path;
}

/* This function runs once at startup. */
SDL_AppResult SDL_AppInit(void** appstate, int argc, char* argv[])
{
    /* Create the window */
    if (!SDL_CreateWindowAndRenderer("Hello World", 1920, 1080, SDL_WINDOW_BORDERLESS | SDL_WINDOW_RESIZABLE, &window, &renderer)) {
        SDL_Log("Couldn't create window and renderer: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }

    const std::string data_path = GetDataPath().string();
    const std::string play_path = data_path + "/play.bmp";
    //const SDL_Surface* surface = IMG_Load(play_path.c_str());
    texture = IMG_LoadTexture(renderer, play_path.c_str());
    const auto msg = SDL_GetError();

    return SDL_APP_CONTINUE;
}

/* This function runs when a new event (mouse input, keypresses, etc) occurs. */
SDL_AppResult SDL_AppEvent(void* appstate, SDL_Event* event)
{
    if (event->type == SDL_EVENT_KEY_DOWN && event->key.key == SDLK_ESCAPE) {
        return SDL_APP_SUCCESS;  /* end the program, reporting success to the OS. */
    }
    if (event->type == SDL_EVENT_QUIT) {
        return SDL_APP_SUCCESS;  /* end the program, reporting success to the OS. */
    }
    return SDL_APP_CONTINUE;
}

/* This function runs once per frame, and is the heart of the program. */
SDL_AppResult SDL_AppIterate(void* appstate){
    const char* message = "Hello World!";
    int w = 0, h = 0;
    float x, y;
    const float scale = 4.0f;

    /* Center the message and scale it up */
    SDL_GetRenderOutputSize(renderer, &w, &h);
    SDL_SetRenderScale(renderer, scale, scale);
    x = ((w / scale) - SDL_DEBUG_TEXT_FONT_CHARACTER_SIZE * SDL_strlen(message)) / 2;
    y = ((h / scale) - SDL_DEBUG_TEXT_FONT_CHARACTER_SIZE) / 2;

    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
    SDL_RenderClear(renderer);
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);

    /* Draw image */
    SDL_RenderTexture(renderer, texture, nullptr, nullptr);

    /* Draw the message */
    SDL_RenderDebugText(renderer, x, y, message);

    /* Display */
    SDL_RenderPresent(renderer);

    return SDL_APP_CONTINUE;
}

/* This function runs once at shutdown. */
void SDL_AppQuit(void* appstate, SDL_AppResult result)
{
}
