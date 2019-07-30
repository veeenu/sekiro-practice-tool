#define UNICODE
#include <MinHook.h>
#include <Windows.h>
#include <d3d11.h>
#include <dxgi.h>
#include <fstream>
#include <imgui.h>
#include <iostream>
#include <string>
#include <thread>
#include "imgui/imgui_impl_dx11.h"
#include "imgui/imgui_impl_win32.h"
#include "ui.h"
#include "config.h"

typedef HRESULT(__fastcall *IDXGISwapChainPresent)(IDXGISwapChain *pSwapChain,
                                                   UINT SyncInterval,
                                                   UINT Flags);
IDXGISwapChainPresent fnIDXGISwapChainPresent;

extern LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg,
                                              WPARAM wParam, LPARAM lParam);

void MakeConsole() {
  AllocConsole();
  SetConsoleTitle(L"Sekiro Practice Tool DLL by johndisandonato");
  freopen("CONOUT$", "w", stdout);
  freopen("CONOUT$", "w", stderr);
  freopen("CONIN$", "r", stdin);
}

HRESULT GetDeviceAndCtxFromSwapchain(IDXGISwapChain *pSwapChain,
                                     ID3D11Device **ppDevice,
                                     ID3D11DeviceContext **ppContext) {
  HRESULT ret =
      pSwapChain->GetDevice(__uuidof(ID3D11Device), (PVOID *)ppDevice);

  if (SUCCEEDED(ret))
    (*ppDevice)->GetImmediateContext(ppContext);

  return ret;
}

BOOL initialized = false;
ID3D11DeviceContext *pContext = NULL;
ID3D11Device *pDevice = NULL;
ID3D11RenderTargetView *mainRenderTargetView;
static IDXGISwapChain *pSwapChain = NULL;
static WNDPROC OriginalWndProcHandler = nullptr;
HWND window = nullptr;
long i = 0;

LRESULT CALLBACK hWndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
  ImGuiIO &io = ImGui::GetIO();
  POINT mPos;
  GetCursorPos(&mPos);
  ScreenToClient(window, &mPos);
  ImGui::GetIO().MousePos.x = mPos.x;
  ImGui::GetIO().MousePos.y = mPos.y;

  ImGui_ImplWin32_WndProcHandler(hWnd, uMsg, wParam, lParam);

  return CallWindowProc(OriginalWndProcHandler, hWnd, uMsg, wParam, lParam);
}

HRESULT __fastcall Present(IDXGISwapChain *pChain, UINT SyncInterval,
                           UINT Flags) {
  if (!initialized) {
    std::cout << "Initializing DirectX" << std::endl;
    if (FAILED(GetDeviceAndCtxFromSwapchain(pChain, &pDevice, &pContext)))
      return fnIDXGISwapChainPresent(pChain, SyncInterval, Flags);

    ImGui::CreateContext();
    ImGuiIO &io = ImGui::GetIO();
    (void)io;

    DXGI_SWAP_CHAIN_DESC sd;
    pChain->GetDesc(&sd);
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
    io.IniFilename = nullptr;

    window = sd.OutputWindow;
    OriginalWndProcHandler =
        (WNDPROC)SetWindowLongPtr(window, GWLP_WNDPROC, (LONG_PTR)hWndProc);

    ImGui_ImplWin32_Init(window);
    ImGui_ImplDX11_Init(pDevice, pContext);
    ImGui::GetIO().ImeWindowHandle = window;

    ID3D11Texture2D *pBackBuffer;

    pChain->GetBuffer(0, __uuidof(ID3D11Texture2D), (LPVOID *)&pBackBuffer);
    pDevice->CreateRenderTargetView(pBackBuffer, NULL, &mainRenderTargetView);
    pBackBuffer->Release();

    initialized = true;
  }

  ImGui_ImplWin32_NewFrame();
  ImGui_ImplDX11_NewFrame();

  ImGui::NewFrame();

  // bool bShow = true;
  // ImGui::ShowDemoWindow(&bShow);
  UI::Instance().Render();

  ImGui::EndFrame();

  ImGui::Render();

  pContext->OMSetRenderTargets(1, &mainRenderTargetView, NULL);
  ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());

  return fnIDXGISwapChainPresent(pChain, SyncInterval, Flags);
}

DWORD WINAPI run_thread(LPVOID param) {
  MakeConsole();

  auto cfg = Config::Instance();
  auto s_true = std::string("true");
  std::cout << "Setting: " << cfg.setting("enabled") << std::endl;
  if (cfg.setting("enabled") != s_true) {
    return 0;
  }

  if (cfg.setting("debug") == s_true) {
    // unimplemented
  }

  std::cout << "Hooking functions..." << std::endl;

  DWORD_PTR hDxgi = (DWORD_PTR)GetModuleHandle(L"dxgi.dll");

  LPVOID fnInitAddr = reinterpret_cast<LPVOID>(
      (IDXGISwapChainPresent)((DWORD_PTR)hDxgi + 0x5070));

  MH_Initialize();
  MH_CreateHook(fnInitAddr, &Present,
                reinterpret_cast<LPVOID *>(&fnIDXGISwapChainPresent));
  MH_EnableHook(fnInitAddr);

  return 0;
}

extern "C" {
  int __declspec(dllexport) __stdcall postAttach() {
    DWORD tmp;
    CreateThread(NULL, 0, run_thread, NULL, 0, &tmp);
    return 0;
  }
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call,
                      LPVOID lpReserved) {
  return TRUE;
}
