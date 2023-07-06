module;

#include "framework.h"
#include "Resource.h"
#include <string>
#include <array>

import SharedMemoryManager;

export module ExtendTimeDialog;

using namespace rozen::sharedmemory;

namespace rozen::timedialog
{
    static std::array timeStrings {L"5 minutes", L"10 minutes", L"15 minutes", L"30 minutes", L"1 hour"};
    static std::array timeValues {5, 10, 15, 30, 60};

    constexpr auto MAX_SHARED_MEMORY_SIZE = 1024;
    constexpr auto SHARED_MEMORY_NAME = L"Global\\BratLockerSharedMemory";

    BOOL CALLBACK DialogProc(HWND hwndDlg, UINT uMsg, WPARAM wParam, LPARAM lParam);

    export class ExtendTimeDialog
    {
    public:

        ExtendTimeDialog() = default;
        ExtendTimeDialog(const ExtendTimeDialog&) = delete;
        ExtendTimeDialog(ExtendTimeDialog&&) = delete;
        ~ExtendTimeDialog() = default;

        ExtendTimeDialog& operator=(const ExtendTimeDialog&) = delete;
        ExtendTimeDialog& operator=(ExtendTimeDialog&&) = delete;

        bool isUnblockEvent()
        {
            SharedMemoryManager manager;
            auto ret = false;

            if (manager.open(SHARED_MEMORY_NAME, MAX_SHARED_MEMORY_SIZE))
            {
                manager.lock();
                
                auto str = static_cast<const wchar_t*>(manager.getBuffer());
                std::wstring result(str);

                if (result == L"unblock:")
				{
                    manager.clear();
					ret = true;
				}

                manager.release();
                manager.close();
            }

            return ret;
        }

        void show(HWND parent, std::wstring& pass)
        {
            if (m_hWnd != nullptr)
            {
                return;
            }

            this->pass = pass;

            HINSTANCE hInstance = GetModuleHandle(nullptr);
            m_hWnd = CreateDialogParam(hInstance, MAKEINTRESOURCE(IDD_LOCK_DIALOG), parent, (DLGPROC)DialogProc, reinterpret_cast<LPARAM>(this));
            ShowWindow(m_hWnd, SW_SHOW);
        }

        auto setAsClosed() noexcept
        {
            m_hWnd = nullptr;
        }

        auto getHwnd() const noexcept
        {
            return m_hWnd;
        }

        auto close() noexcept
        {
            if (m_hWnd != nullptr)
            {
                DestroyWindow(m_hWnd);
                setAsClosed();
            }
        }

        auto getPassword() -> std::wstring&
        {
			return pass;
		}

    private:

        HWND m_hWnd = nullptr;
        std::wstring pass;
    };

    static ExtendTimeDialog *dialog = nullptr;

    BOOL CALLBACK DialogProc(HWND hwndDlg, UINT uMsg, WPARAM wParam, LPARAM lParam)
    {
        switch (uMsg)
        {
        case WM_INITDIALOG:
            // Initialize the dialog here
            dialog = reinterpret_cast<ExtendTimeDialog*>(lParam);
            for (auto i = 0; i < timeStrings.size(); ++i)
            {
                SendDlgItemMessage(hwndDlg, IDC_COMBO_ADDITIONAL_TIME, CB_ADDSTRING, 0, (LPARAM)timeStrings[i]);
            }
            SendDlgItemMessage(hwndDlg, IDC_COMBO_ADDITIONAL_TIME, CB_SETCURSEL, 0, 0);
            break;

        case WM_COMMAND:
            if (LOWORD(wParam) == IDOK)
            {
                constexpr auto MAX_PASSWORD_LENGTH = 256;

                //get the password from edit box
                wchar_t password[MAX_PASSWORD_LENGTH];
                GetDlgItemText(hwndDlg, IDC_EDIT_PASSWORD, password, MAX_PASSWORD_LENGTH);

                std::wstring enteredPassword(password);

                if (enteredPassword.empty())
                {
					MessageBox(hwndDlg, L"Please enter the parent's password to unlock the computer!", L"Ups", MB_OK);
					break;
				}

                if (enteredPassword == dialog->getPassword())
                {
					auto index = SendDlgItemMessage(hwndDlg, IDC_COMBO_ADDITIONAL_TIME, CB_GETCURSEL, 0, 0);
					auto value = timeValues[index];

                    SharedMemoryManager manager;

                    if (manager.open(SHARED_MEMORY_NAME, MAX_SHARED_MEMORY_SIZE))
                    {
                        manager.lock();
                        std::wstring str(L"addtime:");
                        str += std::to_wstring(value);
                        manager.write(str);
                        manager.release();
                        manager.close();
                    }
                    else
                    {
                        MessageBox(hwndDlg, L"Failed to open shared memory!", L"Error", MB_OK | MB_ICONERROR);
                    }

					EndDialog(hwndDlg, value);
				}
                else
                {
					MessageBox(hwndDlg, L"Wrong password!", L"Ups", MB_OK);
                    break;
				}

                dialog->setAsClosed();
                EndDialog(hwndDlg, 0);
            }
            else if (LOWORD(wParam) == IDCANCEL)
            {
                dialog->setAsClosed();
                EndDialog(hwndDlg, -1);
            }
            break;

        case WM_CLOSE:
            EndDialog(hwndDlg, 0);
            return TRUE;

        default:
            break;
        }

        return FALSE;
    }
}
