#include <windows.h>
#include <wtsapi32.h>

int runAsUser(const char *user_name, const char *password, const char *domain, const char *program)
{
    HANDLE hToken;
    LPCTSTR lpApplicationName = program;
    LPTSTR lpCommandLine = NULL;
    LPSECURITY_ATTRIBUTES lpProcessAttributes = NULL;
    LPSECURITY_ATTRIBUTES lpThreadAttributes = NULL;
    BOOL bInheritHandles = FALSE;
    DWORD dwCreationFlags = CREATE_NEW_CONSOLE;
    LPVOID lpEnvironment = NULL;
    LPCTSTR lpCurrentDirectory = NULL;
    STARTUPINFO si;
    PROCESS_INFORMATION pi;

    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    si.wShowWindow = FALSE;
    
    ZeroMemory(&pi, sizeof(pi));

    DWORD sessionId = WTSGetActiveConsoleSessionId();
    if(sessionId == 0xFFFFFFFF)
    {
        return -1;
    }

    if(!WTSQueryUserToken(sessionId, &hToken))
    {
        return GetLastError();
    }
    
    // Create the process as the user
    if (!CreateProcessAsUser(hToken, lpApplicationName, lpCommandLine, lpProcessAttributes, lpThreadAttributes, bInheritHandles, dwCreationFlags, lpEnvironment, lpCurrentDirectory, &si, &pi))
    {
        CloseHandle(hToken);
        return GetLastError();
    }

    WaitForSingleObject(pi.hProcess, INFINITE);

    // Close the handles
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
    CloseHandle(hToken);

    return 0;
}
