using CSharp.Products;

namespace CSharp.Creators
{
    public class HtmlDialog: Dialog
    {
        protected override IButton CreateButton()
        {
            return new HtmlButton();
        }
    }
}