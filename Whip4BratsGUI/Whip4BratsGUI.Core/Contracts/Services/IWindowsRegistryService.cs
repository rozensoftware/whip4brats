using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Contracts.Services;
public interface IWindowsRegistryService
{
    PlayCalendar ReadPlayTime();
    bool WritePlayTime(PlayCalendar playTimes);
    bool CheckIfRegistryKeyExists();
    bool IsDisabled();
    void SetDisabled(bool disabled);
    void InitializeRegistrySettings();
    void UpdateCredentials(string parentPassword, string childUserName, string childPassword);
    void ReadCredentials(out string parentPassword, out string childUserName, out string childPassword);
}
