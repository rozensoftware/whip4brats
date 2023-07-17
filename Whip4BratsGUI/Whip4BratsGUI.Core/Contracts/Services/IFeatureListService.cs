using Whip4BratsGUI.Core.Models;

namespace Whip4BratsGUI.Core.Contracts.Services;

public interface IFeatureListService
{
    Task<IEnumerable<Feature>> GetContentGridAsync();
}
