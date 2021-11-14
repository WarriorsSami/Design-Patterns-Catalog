using CSharp.Products;

namespace CSharp.Creators
{
    public class WindowsDialog: Dialog
    {
        protected override IButton CreateButton()
        {
            return new WindowsButton();
        }
    }
}