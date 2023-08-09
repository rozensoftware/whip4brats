namespace Whip4BratsGUI.Core.Contracts.Services;
public interface IAuxiliaryService
{
    IList<string> GetWeekDays();
    void Initialize();
    void SetParentLogged(bool isLogged);
    void RunExternalProgram(string programPath);
    void StartService(string serviceName);
    string DecodeFromBase64(string encodedData);
    string GetProgramPath();
    bool IsParentLogged();
    bool IsServiceRunning(string serviceName);
}
