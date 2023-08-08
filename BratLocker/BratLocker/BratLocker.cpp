// BratLocker.cpp : Defines the entry point for the application.
//

#include "framework.h"
#include "BratLocker.h"
#include <string>

#define IDT_EYETIMER    WM_USER

constexpr auto INTERVAL_TIME_MS = 3000;
constexpr auto CLASS_NAME = L"BratLocker";
constexpr auto WINDOW_NAME = L"BratLocker";

// Global Variables:
HINSTANCE hInst;                                // current instance
HANDLE hMutexOneInstance;
std::wstring password;

// Forward declarations of functions included in this code module:
ATOM                MyRegisterClass(HINSTANCE hInstance);
BOOL                InitInstance(HINSTANCE, int);
LRESULT CALLBACK    WndProc(HWND, UINT, WPARAM, LPARAM);
INT_PTR CALLBACK    About(HWND, UINT, WPARAM, LPARAM);

import ExtendTimeDialog;

using namespace rozen::timedialog;

ExtendTimeDialog timeDialog;

int APIENTRY wWinMain(_In_ HINSTANCE hInstance,
                     _In_opt_ HINSTANCE hPrevInstance,
                     _In_ LPWSTR    lpCmdLine,
                     _In_ int       nCmdShow)
{
    UNREFERENCED_PARAMETER(hPrevInstance);

    password = lpCmdLine;

    hMutexOneInstance = CreateMutex(NULL, TRUE,
        _T("B503CF9C-6A01-4351-BBC5-D25FCFF45291"));

    if ((GetLastError() == ERROR_ALREADY_EXISTS))
    {
        return FALSE;
    }

    MyRegisterClass(hInstance);

    // Perform application initialization:
    if (!InitInstance (hInstance, nCmdShow))
    {
        return FALSE;
    }

    MSG msg;

    // Main message loop:
    while (GetMessage(&msg, nullptr, 0, 0))
    {
        auto dialogHwnd = timeDialog.getHwnd();

        if (!IsWindow(dialogHwnd) || !IsDialogMessage(dialogHwnd, &msg))
        {
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
    }

    return (int) msg.wParam;
}



//
//  FUNCTION: MyRegisterClass()
//
//  PURPOSE: Registers the window class.
//
ATOM MyRegisterClass(HINSTANCE hInstance)
{
    WNDCLASSEXW wcex;

    wcex.cbSize = sizeof(WNDCLASSEX);

    wcex.style          = 0;
    wcex.lpfnWndProc    = WndProc;
    wcex.cbClsExtra     = 0;
    wcex.cbWndExtra     = 0;
    wcex.hInstance      = hInstance;
    wcex.hIcon          = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_BRATLOCKER));
    wcex.hCursor        = LoadCursor(nullptr, IDC_ARROW);
    wcex.hbrBackground  = (HBRUSH)(COLOR_WINDOW+1);
    wcex.lpszMenuName   = NULL;
    wcex.lpszClassName  = CLASS_NAME;
    wcex.hIconSm        = LoadIcon(wcex.hInstance, MAKEINTRESOURCE(IDI_SMALL));

    return RegisterClassExW(&wcex);
}

BOOL InitInstance(HINSTANCE hInstance, int nCmdShow)
{
   hInst = hInstance; // Store instance handle in our global variable

   HWND hWnd = CreateWindow(CLASS_NAME, WINDOW_NAME, WS_POPUP,
      0, 0, GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN), nullptr, nullptr, hInstance, nullptr);

   if (!hWnd)
   {
      return FALSE;
   }

   if (!SetTimer(hWnd,             // handle to main window 
       IDT_EYETIMER,               // timer identifier 
       INTERVAL_TIME_MS,           // interval (1min)
       (TIMERPROC)NULL))           // no timer callback 
   {
       return FALSE;
   }

   ShowWindow(hWnd, nCmdShow);
   UpdateWindow(hWnd);
    
   return TRUE;
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam)
{
    switch (message)
    {
    case WM_LBUTTONDOWN:
        if (timeDialog.getHwnd() == nullptr)
        {
            timeDialog.show(hWnd, password);
        }
        break;

    case WM_TIMER:
        if (wParam == IDT_EYETIMER)
        {
            if (timeDialog.getHwnd() == nullptr)
            {
                SetWindowPos(hWnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW);
            }

            if (timeDialog.isUnblockEvent())
            {
                SendMessage(hWnd, WM_CLOSE, 0, 0);
            }
        }
        break;

    case WM_PAINT:
    {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hWnd, &ps);

        RECT rcClient;
        GetClientRect(hWnd, &rcClient);
        int cx = rcClient.right / 2;
        int cy = rcClient.bottom / 2;

        HBRUSH greybrush = CreateSolidBrush(RGB(192, 192, 192));
        HBRUSH oldbrush = (HBRUSH)SelectObject(hdc, greybrush);

        FillRect(hdc, &rcClient, greybrush);
        SelectObject(hdc, oldbrush);

        HFONT newfont = CreateFont(32, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE, DEFAULT_CHARSET, OUT_OUTLINE_PRECIS,
            CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, FF_DONTCARE, L"Segoe UI");
        
        HFONT oldfont = (HFONT)SelectObject(hdc, newfont);
        
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, 0x00000000);

        DrawText(hdc, L"Time out!", -1, &rcClient, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

        SelectObject(hdc, oldfont);
        
        DeleteObject(greybrush);
        DeleteObject(newfont);

        EndPaint(hWnd, &ps);
    }
    break;

    //Add on close handler to decide whether to close the window or not
    case WM_CLOSE:
		if (timeDialog.getHwnd() == nullptr)
		{
			DestroyWindow(hWnd);
		}
		break;

    case WM_DESTROY:
        timeDialog.close();
        ReleaseMutex(hMutexOneInstance);
        KillTimer(hWnd, IDT_EYETIMER);
        PostQuitMessage(0);
        break;

    default:
        return DefWindowProc(hWnd, message, wParam, lParam);
    }
    return 0;
}
