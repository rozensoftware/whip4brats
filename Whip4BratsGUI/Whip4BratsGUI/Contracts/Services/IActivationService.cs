namespace Whip4BratsGUI.Contracts.Services;

public interface IActivationService
{
    Task ActivateAsync(object activationArgs);
}
