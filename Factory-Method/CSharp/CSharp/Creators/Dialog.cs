using CSharp.Products;

namespace CSharp.Creators
{
    public abstract class Dialog
    {
        protected abstract IButton CreateButton();

        public void RenderWindow()
        {
            var button = CreateButton();
            button.Render();
        }
    }
}