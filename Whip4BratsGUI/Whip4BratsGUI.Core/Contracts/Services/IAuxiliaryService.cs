using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Contracts.Services;
public interface IAuxiliaryService
{
    IList<string> GetWeekDays();
    void Initialize();
    void SetParentLogged(bool isLogged);
    string DecodeFromBase64(string encodedData);
    bool IsParentLogged();
}
