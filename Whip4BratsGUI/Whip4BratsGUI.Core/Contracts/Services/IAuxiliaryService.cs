using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Contracts.Services;
public interface IAuxiliaryService
{
    IList<string> GetWeekDays();
    void Initialize();
}
