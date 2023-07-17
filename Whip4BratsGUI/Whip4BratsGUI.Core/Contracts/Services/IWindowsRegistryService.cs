using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Contracts.Services;
public interface IWindowsRegistryService
{
    PlayCalendar ReadPlayTime();
    bool WritePlayTime(PlayCalendar playTimes);
}
